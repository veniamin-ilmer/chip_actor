use stakker::*;

//Faraday FE2010A - PC Bus, CPU, and Peripheral Controller
//https://github.com/skiselev/micro_8088/blob/master/Documentation/Faraday-XT_Controller-FE2010A.md

use log::debug;

//Configuration Register
#[derive(Debug, Default)]
enum CPUSpeed {
  #[default]
  MHz477,
  MHz715,
  MHz954,
}

//Control Register
#[derive(Debug, Default)]
struct Enable {
  timer_2: bool,
  speaker: bool,
  parity_check: bool,
  io_check: bool,
  keyboard_clock: bool,
  nmi: bool,
  nmi_8087: bool,
  lock_register: bool,  //I am saving this, but not locking anything.
}

#[derive(Debug, Default)]
enum SwitchSelect {
  #[default]
  S0, S1
}

#[derive(Debug, Default)]
enum NumOfFloppies {
  #[default]
  N0, N1, N2, N3,
}

#[derive(Debug, Default)]
enum MemorySize {
  #[default]
  K640, K512, K256,
}

//Switch Register
#[derive(Debug, Default)]
struct Switches {
  installed_8087: bool,
  memory_size: MemorySize,
  num_of_floppies: NumOfFloppies,
  switch_select: SwitchSelect,
  cpu_speed: CPUSpeed,
}

//Part of Switch Register
#[derive(Debug, Default)]
struct Errors {
  io_check: bool,
  parity_check: bool,
}

#[derive(Debug, Default)]
pub(crate) struct PPI {
  enable: Enable,
  switches: Switches,
  errors: Errors,
  keyboard_character: u8,
}

impl PPI {

  pub(crate) fn init(_: CX![]) -> Option<PPI> {
    Some(Default::default())
  }
  pub(crate) fn set_configuration(&mut self, _: CX![], value: u8) {
    self.enable.parity_check = matches!(value & 0b1, 0);
    self.enable.nmi_8087 = matches!(value & 0b10, 0b10);
    self.switches.memory_size = match ((value >> 2) & 1, (value >> 4) & 1) {
      (0, 0) => MemorySize::K640,
      (0, 1) => MemorySize::K512,
      (1, 0) => MemorySize::K256,
      _ => unimplemented!("Unknown memory size requested."),
    };
    self.switches.cpu_speed = match value >> 5 {
      0 | 1 => CPUSpeed::MHz477,
      2 | 3 => CPUSpeed::MHz715,
      _     => CPUSpeed::MHz954,
    };
    debug!("parity_check enabled: {}, 8087 NMI Enabled: {}, Memory Size: {:?}, CPU Speed: {:?}",
    self.enable.parity_check, self.enable.nmi_8087, self.switches.memory_size, self.switches.cpu_speed);
  }
  
  pub(crate) fn set_nmi(&mut self, _: CX![], value: u8) {
    self.enable.nmi = matches!(value & 0b1000_0000, 0b1000_0000);
    if self.enable.nmi { debug!("NMI Enabled"); } else { debug!("NMI Disabled"); }
  }
  
  pub(crate) fn write_port_a(&self, _: CX![], value: u8) {
    debug!("Port A received {:X}", value);
  }
  
  //Control Register
  pub(crate) fn write_port_b(&mut self, _: CX![], value: u8) {
    self.enable.timer_2 = matches!(value & 0b1, 0b1);
    self.enable.speaker = matches!(value & 0b10, 0b10);
    self.switches.switch_select = if matches!(value & 0b100, 0b100) { SwitchSelect::S1 } else { SwitchSelect::S0 }; //NOTE - In XT this is 0b100. In PC this is 0b1000.
    self.enable.parity_check = matches!(value & 0b1_0000, 0); //Note it is reversed here.
    self.enable.io_check = matches!(value & 0b10_0000, 0); //Note it is reversed here.
    self.enable.keyboard_clock = matches!(value & 0b100_0000, 0b100_0000);
    if matches!(value & 0b1000_0000, 0b1000_0000) { //Clear Keyboard Data Register
      self.keyboard_character = 0;
    }
    debug!("Write {:?}", self.enable);
  }
  pub(crate) fn read_port_a(&self, _: CX![], ret: Ret<u8>) {
    ret!([ret], self.keyboard_character)
  }
  pub(crate) fn read_port_b(&self, _: CX![], ret: Ret<u8>) {
    let mut result = 0;
    if self.enable.timer_2 { result |= 0b1 };
    if self.enable.speaker { result |= 0b10 };
    if let SwitchSelect::S1 = self.switches.switch_select { //NOTE - In XT this is 0b100. In PC this is 0b1000.
      result |= 0b100;
    }
    if !self.enable.parity_check { result |= 0b1_0000 }; //Note it is reversed here.
    if !self.enable.io_check { result |= 0b10_0000 }; //Note it is reversed here.
    if self.enable.keyboard_clock { result |= 0b100_0000 };
    debug!("Read {:?}", self.enable);
    ret!([ret], result)
  }
  pub(crate) fn read_port_c(&self, _: CX![], ret: Ret<u8>) {
    let mut result = 0;
    match self.switches.switch_select {
      SwitchSelect::S0 => {
        result |= (match self.switches.num_of_floppies {
          NumOfFloppies::N0 => 0,
          NumOfFloppies::N1 => 1,
          NumOfFloppies::N2 => 2,
          NumOfFloppies::N3 => 3,
        } << 2);
      },
      SwitchSelect::S1 => {
        if self.switches.installed_8087 { result |= 0b10 }
        result |= (match self.switches.memory_size {
          MemorySize::K640 => 0b00,
          MemorySize::K512 => 0b10,
          MemorySize::K256 => 0b01,
        } << 2);
      }
    }
    //TODO - Timer 2 output
    if self.errors.io_check { result |= 0b100_0000 }
    if self.errors.parity_check { result |= 0b1000_0000 }
    debug!("Num of Floppies: {:?}, Memory Size: {:?}, CPU Speed: {:?}, 8087 installed: {}, IO Check: {}, Parity Check: {}",
    self.switches.num_of_floppies, self.switches.memory_size, self.switches.cpu_speed, self.switches.installed_8087, self.errors.io_check, self.errors.parity_check);
    ret!([ret], result)
  }
}