use stakker::*;
use crate::Board;

mod definitions;
mod instructions;

use definitions::memory::Memory;
use definitions::memory::Segment;
use definitions::register::Registers;
use definitions::flag::Flags;
use definitions::operand;

use log::{trace, debug};

pub(crate) struct CPU {
  pub(crate) board: Actor<Board>,
  pub(crate) memory: Memory,
  pub(crate) regs: Registers,
  pub(crate) flags: Flags,
  pub(crate) current_address: usize,
  pub(crate) logging: bool,
}

impl CPU {
  
  pub(crate) fn init(_: CX![], board: Actor<Board>, mut bios_rom: Vec<u8>, mut video_rom: Vec<u8>, mut disk_rom: Vec<u8>) -> Option<Self> {
    let mut memory = Memory {
      cs: 0xF000,
      ds: 0,
      ss: 0,
      es: 0,
      ip: 0xFFF0,
      current_segment: Segment::DS,
      current_instruction: 0,
      ram: vec![0u8; 0xC_0000],
    };
    
    memory.ram.append(&mut video_rom);
    memory.ram.resize(0xC_8000, 0); //Fill up the remainder of space up to the bios.
    memory.ram.append(&mut disk_rom);
    memory.ram.resize(0xF_0000, 0); //Fill up the remainder of space up to the bios.
    
    if bios_rom.len() != 0x1_0000 {
      panic!("The ROM size is wrong: {:X}. It must be size 0x10000.", bios_rom.len());
    }
    memory.ram.append(&mut bios_rom);

    let current_address = memory.get_current_address();

    Some(CPU {
      board,
      memory,
      current_address,
      regs: Default::default(),
      flags: Default::default(),
      logging: false,
    })
  }

  pub(crate) fn single_run(&mut self, cx: CX![]) {
    self.memory.prepare_next_instruction();
    let cycles = instructions::lookup::run_next_instruction(self, cx.this());
    let duration = std::time::Duration::from_nanos(cycles as u64 * 210); //4.77 Mhz = 210 nanosecond delay.
    let instant = cx.now() + duration;
    //self.print_registers();
    at!(instant, [cx], single_run());
  }

  pub(crate) fn set_al(&mut self, _: CX![], result: u8) {
    self.regs.set_byte(&definitions::register::Byte::AL, result);
  }
  pub(crate) fn set_ax(&mut self, _: CX![], result: u16) {
    self.regs.set_word(&definitions::register::Word::AX, result);
  }

  pub(crate) fn print_registers(&self) {
    debug!("AX={:04X}  BX={:04X}  CX={:04X}  DX={:04X}  SP={:04X}  BP={:04X}  SI={:04X}  DI={:04X}",
             self.regs.ax, self.regs.bx, self.regs.cx, self.regs.dx, self.regs.sp, self.regs.bp, self.regs.si, self.regs.di);
    debug!("DS={:04X}  ES={:04X}  SS={:04X}  CS={:04X}  IP={:04X} C={} P={} A={} Z={} S={} O={}",
             self.memory.ds, self.memory.es, self.memory.ss, self.memory.cs, self.memory.ip, self.flags.carry, self.flags.parity, self.flags.adjust, self.flags.zero, self.flags.sign, self.flags.overflow);
  }

  pub(crate) fn read_byte(&mut self, op: &operand::Byte) -> u8 {
    match op {
      operand::Byte::Mem{addr, ..} => self.memory.get_byte(*addr),
      operand::Byte::Reg(reg) => self.regs.get_byte(reg),
      operand::Byte::Imm(imm) => *imm,
    }
  }
  pub(crate) fn read_word(&mut self, op: &operand::Word) -> u16 {
    match op {
      operand::Word::Mem{addr, ..} => self.memory.get_word(*addr),
      operand::Word::Reg(reg) => self.regs.get_word(reg),
      operand::Word::Seg(seg) => self.memory.get_seg(seg),
      operand::Word::Imm(imm) => *imm,
    }
  }

  pub(crate) fn write_byte(&mut self, op: &operand::Byte, value: u8) {
    match op {
      operand::Byte::Mem{addr, ..} => self.memory.set_byte(*addr, value),
      operand::Byte::Reg(reg) => self.regs.set_byte(reg, value),
      operand::Byte::Imm(_) => panic!("Attemped write to imm."),
    };
  }
  pub(crate) fn write_word(&mut self, op: &operand::Word, value: u16) {
    match op {
      operand::Word::Mem{addr, ..} => self.memory.set_word(*addr, value),
      operand::Word::Reg(reg) => self.regs.set_word(reg, value),
      operand::Word::Seg(seg) => self.memory.set_seg(seg, value),
      operand::Word::Imm(_) => panic!("Attemped write to imm."),
    };
  }

  pub(crate) fn interrupt(&mut self, _: CX![], int_index: u8) {
    instructions::jump::hardware_int(self, int_index as u8);
  }
}
