use stakker::*;

use log::debug;

pub(crate) struct IBM_XT {
  cpu: Actor<crate::CPU>,
  pit: Actor<crate::Timer>,
  ppi: Actor<crate::PPI>,
  graphics: Actor<crate::Graphics>,
  dma: Actor<crate::MemoryController>,
  pic: Actor<crate::PIC>,
}

impl IBM_XT {
  pub(crate) fn init(_: CX![],
      cpu: Actor<crate::CPU>,
      pit: Actor<crate::Timer>,
      ppi: Actor<crate::PPI>,
      graphics: Actor<crate::Graphics>,
      dma: Actor<crate::MemoryController>,
      pic: Actor<crate::PIC>,
    ) -> Option<Self> {
    Some(Self {
      cpu,
      pit,
      ppi,
      graphics,
      dma,
      pic
    })
  }
  
  pub(crate) fn timer_interrupt(&self, _: CX![], select_counter: u8) {
    println!("Timer interrupt goes here for IRQ {}.", select_counter);
  }
  
  pub(crate) fn pic_interrupt(&self, _: CX![], interrupt: u8) {
    println!("Pic interrupt goes here for INT {}.", interrupt);
  }
  
  pub(crate) fn out_byte(&self, _: CX![], port: u16, value: u8) {
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
      0x63 => call!([self.ppi], set_configuration(value)),
      0x83 => debug!("083 - High order 4 bits of DMA channel 1 address {:X}", value),
      0xA0 => call!([self.ppi], set_nmi(value)),
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
      _ => panic!("In port {:X}", port),
    }
  }
  pub(crate) fn in_word(&self, _: CX![], port: u16, ret: Ret<u16>) {
    ret!([ret], 0);
    panic!("16 bit In port {:X}", port);
  }
  
}