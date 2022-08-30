use stakker::*;

use crate::Board;

use log::debug;

pub(crate) struct PPI {
  board: Actor<Board>,
  port_a: u8,
  port_b: u8,
  port_c: u8,
}

impl PPI {

  pub(crate) fn init(_: CX![], board: Actor<Board>) -> Option<PPI> {
    Some(PPI {
      board,
      port_a: 0,
      port_b: 0,
      port_c: 0,
    })
  }
  pub(crate) fn set_configuration(&mut self, _: CX![], value: u8) {
    //Port input vs output and mode would be set here. I don't think this is relevant for emulation
  }
    
  pub(crate) fn write_port_a(&mut self, _: CX![], value: u8) {
    self.port_a = value;
    call!([self.board], ppi_port_a(value));
  }
  
  pub(crate) fn write_port_b(&mut self, _: CX![], value: u8) {
    self.port_b = value;
    call!([self.board], ppi_port_b(value));
  }

  pub(crate) fn write_port_c(&mut self, _: CX![], value: u8) {
    self.port_c = value;
    call!([self.board], ppi_port_c(value));
  }
  
  pub(crate) fn read_port_a(&self, _: CX![], ret: Ret<u8>) {
    ret!([ret], self.port_a)
  }
  pub(crate) fn read_port_b(&self, _: CX![], ret: Ret<u8>) {
    ret!([ret], self.port_b)
  }
  pub(crate) fn read_port_c(&self, _: CX![], ret: Ret<u8>) {
    ret!([ret], self.port_c)
  }
}