use stakker::*;

use log::debug;

#[allow(dead_code)]
#[derive(Debug)]
enum BoardRam {
  K64, K128, K192, K256,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Display {
  None, Color40x25, Color80x25, Monochrome80x25,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Floppies {
  N1, N2, N3, N4,
}

pub(crate) struct IBM_XT {
  cpu: Actor<crate::CPU>,
  pit: Actor<crate::Timer>,
  ppi: Actor<crate::PPI>,
  graphics: Actor<crate::Graphics>,
  dma: Actor<crate::MemoryController>,
  pic: Actor<crate::PIC>,
  fixed_disk: Actor<crate::FixedDisk>,
  
  enable_nmi: bool,
  enable_speaker: bool,
  enable_keyboard: bool,
  
  dip_long_post: bool,
  dip_coprocessor_installed: bool,
  dip_board_ram: BoardRam,
  dip_display: Display,
  dip_floppies: Floppies,
}

impl IBM_XT {
  pub(crate) fn init(_: CX![],
      cpu: Actor<crate::CPU>,
      pit: Actor<crate::Timer>,
      ppi: Actor<crate::PPI>,
      graphics: Actor<crate::Graphics>,
      dma: Actor<crate::MemoryController>,
      pic: Actor<crate::PIC>,
      fixed_disk: Actor<crate::FixedDisk>
    ) -> Option<Self> {
      
    Some(Self {
      cpu,
      pit,
      ppi,
      graphics,
      dma,
      pic,
      fixed_disk,
      
      enable_nmi: false,
      enable_speaker: false,
      enable_keyboard: false,
      dip_long_post: false,
      dip_coprocessor_installed: true,
      dip_board_ram: BoardRam::K256,
      dip_display: Display::Monochrome80x25,
      dip_floppies: Floppies::N1,
    })
  }
  
  fn set_nmi(&mut self, value: u8) {
    self.enable_nmi = matches!(value & 0b1000_0000, 0b1000_0000);
    if self.enable_nmi { debug!("NMI Enabled"); } else { debug!("NMI Disabled"); }
  }

  //Keyboard input / diagnostic output
  pub(crate) fn ppi_port_a(&mut self, _: CX![], _: u8) {
  }
  
  pub(crate) fn ppi_port_b(&mut self, _: CX![], value: u8) {
    let speaker_timer = matches!(value & 0b1, 0b1);
    let speaker = matches!(value & 0b10, 0b10);
    self.enable_speaker = speaker_timer & speaker;
    self.enable_keyboard = matches!(value & 0b100_0000, 0b100_0000);
    
    let mut port_c_val = 0;
    if speaker_timer { port_c_val |= 0b10_0000 }
    
    if matches!(value & 0b100, 0b100) { //NOTE - In XT this is 0b100. In PC this is 0b1000.
      //High switch
      port_c_val |= match self.dip_display {
        Display::None => 0,
        Display::Color40x25 => 1,
        Display::Color80x25 => 2,
        Display::Monochrome80x25 => 3,
      };
      port_c_val |= match self.dip_floppies {
        Floppies::N1 => 0,
        Floppies::N2 => 1,
        Floppies::N3 => 2,
        Floppies::N4 => 3,
      } << 2;
      debug!("Display: {:?}, Floppies: {:?}", self.dip_display, self.dip_floppies);
    } else {
      //Low switch
      if self.dip_long_post { port_c_val |= 0b1 }
      if self.dip_coprocessor_installed { port_c_val |= 0b10 }
      port_c_val |= match self.dip_board_ram {
        BoardRam::K64 => 0,
        BoardRam::K128 => 1,
        BoardRam::K192 => 2,
        BoardRam::K256 => 3,
      } << 2;
      debug!("Long Post: {}, Coprocessor Installed: {}, On System Ram: {:?}", self.dip_long_post, self.dip_coprocessor_installed, self.dip_board_ram);
    }
    call!([self.ppi], write_port_c(port_c_val));
    
    if matches!(value & 0b1000_0000, 0b1000_0000) { //Clear Keyboard Data
      call!([self.ppi], write_port_a(0));
    }
    
    debug!("Speaker Enabled: {}, Keyboard Enabled: {}", self.enable_speaker, self.enable_keyboard);
  }
  
  pub(crate) fn ppi_port_c(&mut self, _: CX![], _: u8) {
  }

  
  pub(crate) fn timer_interrupt(&self, _: CX![], select_counter: u8) {
    if select_counter == 0 {  //Only IRQ 0 is able to signal the PIC.
      call!([self.pic], interrupt_irq0());
    }
  }
  
  pub(crate) fn fixed_disk_interrupt(&self, _: CX![]) {
    call!([self.pic], interrupt_irq5());
  }
  
  pub(crate) fn pic_interrupt(&self, _: CX![], int_index: u8) {
    call!([self.cpu], interrupt(int_index));
  }
  
  pub(crate) fn out_byte(&mut self, _: CX![], port: u16, value: u8) {
    match port {
      0x00 => call!([self.dma], set_address(0, value)),
      0x01 => call!([self.dma], set_count(0, value)),
      0x02 => call!([self.dma], set_address(1, value)),
      0x03 => call!([self.dma], set_count(1, value)),
      0x04 => call!([self.dma], set_address(2, value)),
      0x05 => call!([self.dma], set_count(2, value)),
      0x06 => call!([self.dma], set_address(3, value)),
      0x07 => call!([self.dma], set_count(3, value)),
      0x08 => call!([self.dma], set_status(value)),
      0x0A => call!([self.dma], set_mask(value)),
      0x0B => call!([self.dma], set_mode(value)),
      0x0C => call!([self.dma], reset_flip_flop()),
      0x0D => call!([self.dma], reset_master()),
      0x0E => call!([self.dma], reset_mask()),
      0x0F => call!([self.dma], set_masks(value)),
      0x20 => call!([self.pic], out_port_1(value)),
      0x21 => call!([self.pic], out_port_2(value)),
      0x40 => call!([self.pit], set_count0(value)),
      0x41 => call!([self.pit], set_count1(value)),
      0x42 => call!([self.pit], set_count2(value)),
      0x43 => call!([self.pit], set_control_word(value)),
      0x60 => call!([self.ppi], write_port_a(value)),
      0x61 => call!([self.ppi], write_port_b(value)),
      0x62 => call!([self.ppi], write_port_c(value)),
      0x63 => call!([self.ppi], set_configuration(value)),
      0x83 => debug!("083 - High order 4 bits of DMA channel 1 address {:X}", value),
      0xA0 => self.set_nmi(value),
      0x210 => debug!("OUT Expansion Card Port - {:X}", value),
      0x320 => call!([self.fixed_disk], send_command(value)),
      0x321 => call!([self.fixed_disk], reset(value)),
      0x322 => call!([self.fixed_disk], pulse(value)),
      0x323 => call!([self.fixed_disk], set_dma_and_interrupt(value)),
      0x327 => debug!("OUT Fixed disk port 0x327. Value: {:X}. Undocumented..", value),
      0x32B => debug!("OUT Fixed disk port 0x32B. Value: {:X}. Undocumented..", value),
      0x32F => debug!("OUT Fixed disk port 0x32F. Value: {:X}. Undocumented..", value),
      0x3B4 => call!([self.graphics], choose_register(value)),
      0x3B5 => call!([self.graphics], set_register_data(value)),
      0x3B8 => call!([self.graphics], set_mode_bw(value)),
      0x3B9 => debug!("Port 3B9 got {}. Don't know what this means!", value),
      0x3D4 => call!([self.graphics], choose_register(value)),
      0x3D5 => call!([self.graphics], set_register_data(value)),
      0x3D8 => call!([self.graphics], set_mode_color(value)),
      0x3D9 => debug!("Port 3D9 got {}. Don't know what this means!", value),
      _ => panic!("Out port {:X}, value: {:X}.", port, value),
    }
  }
  pub(crate) fn out_word(&self, _: CX![], port: u16, value: u16) {
    panic!("16 bit Out port {:X}, value: {:X}.", port, value);
  }
  
  pub(crate) fn in_byte(&self, _: CX![], port: u16, ret: Ret<u8>) {
    match port {
      0x00 => call!([self.dma], get_address(0, ret)),
      0x01 => call!([self.dma], get_count(0, ret)),
      0x02 => call!([self.dma], get_address(1, ret)),
      0x03 => call!([self.dma], get_count(1, ret)),
      0x04 => call!([self.dma], get_address(2, ret)),
      0x05 => call!([self.dma], get_count(2, ret)),
      0x06 => call!([self.dma], get_address(3, ret)),
      0x07 => call!([self.dma], get_count(3, ret)),
      0x08 => call!([self.dma], get_status(ret)),
      0x20 => call!([self.pic], in_port_1(ret)),
      0x21 => call!([self.pic], get_irqs_enabled(ret)),
      0x40 => call!([self.pit], get_count0(ret)),
      0x41 => call!([self.pit], get_count1(ret)),
      0x42 => call!([self.pit], get_count2(ret)),
      0x60 => call!([self.ppi], read_port_a(ret)),
      0x61 => call!([self.ppi], read_port_b(ret)),
      0x62 => call!([self.ppi], read_port_c(ret)),
      0x210 => {debug!("IN Expansion Card Port"); ret!([ret], 0);},
      0x320 => call!([self.fixed_disk], read_status(ret)),
      0x321 => call!([self.fixed_disk], status_register(ret)),
      0x3BA => call!([self.graphics], get_status(ret)),
      _ => panic!("In port {:X}", port),
    }
  }
  pub(crate) fn in_word(&self, _: CX![], port: u16, ret: Ret<u16>) {
    ret!([ret], 0);
    panic!("16 bit In port {:X}", port);
  } 
}
