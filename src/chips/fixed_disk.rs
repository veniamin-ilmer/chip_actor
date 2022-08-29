//http://www.techhelpmanual.com/898-xt_hard_disk_ports.html

use stakker::*;

use log::{debug,trace};

use crate::Board;

pub(crate) struct FixedDisk {
  board: Actor<Board>,
  enable_dma: bool,
  enable_int: bool,
  busy: bool,
  pulsed: bool,
  mode: bool,
  request: bool,
  interrupted: bool,
  
  //Commands are expected to be sent in batches of 6 bytes. IBM calls the the DCB (Data Control Block)
  bcd_count: u8,
  bcd_command: u8,
  bcd_drive: u8,
  bcd_head: u8,
  bcd_sector: u8,
  bcd_cylinder: u16,
  bcd_control: u8,
  bcd_field: u8,
  bcd_microseconds_per_step: u16,
  bcd_disable_retry: bool,
  bcd_dont_retry_read: bool,
}

impl FixedDisk {
  pub(crate) fn init(_: CX![], board: Actor<Board>) -> Option<FixedDisk> {
    Some(FixedDisk {
      board,
      enable_dma: false, enable_int: false,
      busy: false, pulsed: false, mode: false, request: false, interrupted: false,
      bcd_count: 0, bcd_command: 0, bcd_drive: 0, bcd_head: 0, bcd_sector: 0, bcd_cylinder: 0, bcd_control: 0, bcd_field: 0, bcd_microseconds_per_step: 0, bcd_disable_retry: false, bcd_dont_retry_read: false
    })
  }
  
  pub(crate) fn read_status(&self, _: CX![], ret: Ret<u8>) {
    let result = self.bcd_drive << 5;
    //if there is an error with any command earlier, bit 1 would be set.
    ret!([ret], result);
  }
  
  //Port 320 Read/Write data
  /*
  To write a command here, a pulse must be send first.
  Commands are expected to be sent in batches of 6 bytes. IBM calls the the DCB (Data Control Block)
  Byte 0 = Command / Opcode.
  Byte 1 = drive / Head number
  Byte 2 = Cylinder High / Sector Number
  Byte 3 = Cylinder Low
  Byte 4 = Interleave or Block Count
  Byte 5 = Control Field
  */
  pub(crate) fn send_command(&mut self, cx: CX![], value: u8) {
    match self.bcd_count {
      0 => {
        self.bcd_command = value;
        debug!("Got bcd command: {}", self.bcd_command);
      },
      1 => {
        self.bcd_drive = value >> 5;
        self.bcd_head = value & 0b1_1111;
        debug!("Got bcd drive: {} bcd head: {}", self.bcd_drive, self.bcd_head);
      },
      2 => {
        self.bcd_sector = value & 0b11_1111;
        self.bcd_cylinder = ((value as u16) & 0b1100_0000) << 2; //Cylinder high
        debug!("Got bcd sector: {}", self.bcd_sector);
      },
      3 => {
        self.bcd_cylinder |= value as u16;  //Cylinder low
        debug!("Got bcd cylinder: {}", self.bcd_cylinder);
      },
      4 => {
        self.bcd_control = value;
        debug!("Got bcd control/block: {}", self.bcd_control);
      },
      5 => {
        self.bcd_microseconds_per_step = match value & 0b111 {
          0 | 6 | 7 => 3000,
          4 => 200,
          5 => 70,
          _ => unreachable!("Unknown microseconds_per_step specified: {}", value),
        };
        self.bcd_disable_retry = matches!(value & 0b1000_0000, 0b1000_0000);  //For testing
        self.bcd_dont_retry_read = matches!(value & 0b100_0000, 0b100_0000);  //For testing
        debug!("Got bcd field control. Microseconds Per Step: {}, Disable Retry: {}, Retry Read: {}", self.bcd_microseconds_per_step, self.bcd_disable_retry, self.bcd_dont_retry_read);
      },
      _ => unimplemented!("Unknown bcd count {}, value {}", self.bcd_count, value),
    }
    
    self.bcd_count += 1;
    if self.bcd_count == 6 {
      self.run_command(cx);
    }
  }
  
  fn run_command(&self, cx: CX![]) {
    match self.bcd_command {
      0x0 => debug!("test drive ready"),
      0x1 => debug!("Recalibrate"),
      0x3 => debug!("Status"),
      0x4 => debug!("Format Drive"),
      0x5 => debug!("Read Verify"),
      0x6 => debug!("Format Track"),
      0x7 => debug!("Format Bad Track"),
      0x8 => debug!("Read"),
      0xA => debug!("Write"),
      0xB => debug!("Seek"),
      0xC => debug!("Initialize Drive Characteristics (The Followed by 8 additional bytes..)"),
      0xD => debug!("Read ECC Burst Length"),
      0xE => debug!("Read Data from Sector Buffer"),
      0xF => debug!("Write Data to Sector Buffer"),
      0b11100000 => debug!("RAM Diagnostic"),
      0b11100011 => debug!("Drive Diagnostic"),
      0b11100100 => debug!("Controller Interal Diagnostics"),
      0b11100101 => debug!("Read Long Track (Returns 512 bytes plus 4 bytes of ECC data per sector)"),
      0b11100110 => debug!("Write Long (Requires 512 bytes plus 4 bytes of ECC data per sector)"),      
      _ => unreachable!("Reserved command {}", self.bcd_command),
    }
    let duration = std::time::Duration::from_millis(10);  //Disk takes a long time to respond...
    let instant = cx.now() + duration;
    at!(instant, [cx], completed());
  }
  
  fn completed(&mut self, _: CX![]) {
    call!([self.board], fixed_disk_interrupt());
    self.interrupted = true;
  }

  
  //Port 321 Read hard disk status.
  
  //Write Controller reset
  pub(crate) fn reset(&mut self, _: CX![], value: u8) {
    self.request = false;
    self.mode = false;
    self.pulsed = false;
    self.busy = false;
    self.interrupted = false;
    self.bcd_count = 0;
    /*

    debug!("Set Control Register {:X}", value);
    */
    /*Bit 7 Disables the four retries by the controller on all
    disk-access commands. Set this bit only during the
    evaluation of the performance of a disk drive.
    Bit 6 If set to 0 during read commands, a reread is
    attempted when an ECC error occurs. If no error
    occurs during reread, the command will complete
    with no error status. If this bit is set to 1, no reread is
    attempted.*/
    debug!("Reset value: {}", value);
  }
  
  pub(crate) fn status_register(&mut self, _: CX![], ret: Ret<u8>) {
    /*
    Bits 0, 1, 2, 3, 4 , 6 , 7 These bits are set to zero.
    Bit 1
    When set, this bit shows an error has
    occurred during command execution.
    Bit 5
    This bit shows the logical unit number of
    the drive.
    */
    /*
    Bit 0 = Busy
    Bit 1 = command/data
    Bit 2 = Mode
    Bit 3 = Request
    After a pulse, busy = true, command/data=true, mode=false, request=true
    */
    let mut result = 0;
    if self.request     { result |= 0b1; }
    if self.mode        { result |= 0b10; }
    if self.pulsed      { result |= 0b100; }
    if self.busy        { result |= 0b1000; }
    if self.interrupted { result |= 0b10_0000; }
    ret!([ret], result);
    debug!("Get Status");
  }
  
  //Port 322 Write generate controller select pulse
  pub(crate) fn pulse(&mut self, _: CX![], value: u8) {
    //Any OUT to this port enables the controller.  Use before a cmd.
    self.request = true;
    self.mode = false;
    self.pulsed = true;
    self.busy = true;
    self.interrupted = false;
    self.bcd_count = 0;
    debug!("Pulse value: {}", value);
  }
  
  //Port 323 Write to DMA and interrupt mask register
  pub(crate) fn set_dma_and_interrupt(&mut self, _: CX![], value: u8) {
    self.enable_dma = matches!(value & 0b1, 0b1);
    self.enable_int = matches!(value & 0b10, 0b10);
    debug!("DMA enabled: {}, Interrupt enabled: {}", self.enable_dma, self.enable_int);
  }
}