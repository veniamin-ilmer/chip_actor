use stakker::*;

use super::super::CPU;

use super::super::definitions::operand;
use super::super::definitions::register;
use super::super::definitions::memory;

use log::Level::{Trace};
use log::{trace, log_enabled};

pub(crate) fn mov_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: MOV {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let value = cpu.read_byte(&get_op);
  cpu.write_byte(&set_op, value);
  set_op.get_cycles_fast(&get_op)
}
pub(crate) fn mov_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: MOV {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let value = cpu.read_word(&get_op);
  cpu.write_word(&set_op, value);
  set_op.get_cycles_fast(&get_op)
}

pub(crate) fn xchg_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: XCHG {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  cpu.write_byte(&set_op, get_val);
  cpu.write_byte(&get_op, set_val);
  set_op.get_cycles()
}
pub(crate) fn xchg_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: XCHG {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_word(&get_op));
  cpu.write_word(&set_op, get_val);
  cpu.write_word(&get_op, set_val);
  set_op.get_cycles()
}

pub(crate) fn lea_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: LEA {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let value = cpu.read_word(&get_op);
  cpu.write_word(&set_op, value);
  match get_op {
    operand::Word::Mem{cycles, ..} => 2 + cycles,
    _ => unreachable!("Tried to run LES on non-memory operand."),
  }
}

pub(crate) fn les_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: LES {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let value = cpu.read_word(&get_op);
  cpu.write_word(&set_op, value);
  if let operand::Word::Mem{addr, ..} = get_op {
    let val2 = cpu.memory.get_word(addr + 2); //Read next word
    cpu.memory.es = val2;
  } //If it is not memory, then this is undefined behaviour. We don't really care. We just won't set ES.
  match get_op {
    operand::Word::Mem{cycles, ..} => 24 + cycles,
    _ => unreachable!("Tried to run LES on non-memory operand."),
  }
}
pub(crate) fn lds_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: LES {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let value = cpu.read_word(&get_op);
  cpu.write_word(&set_op, value);
  if let operand::Word::Mem{addr, ..} = get_op {
    let val2 = cpu.memory.get_word(addr + 2); //Read next word
    cpu.memory.ds = val2;
  } //If it is not memory, then this is undefined behaviour. We don't really care. We just won't set DS.
  match get_op {
    operand::Word::Mem{cycles, ..} => 24 + cycles,
    _ => unreachable!("Tried to run LES on non-memory operand."),
  }
}

//Translate byte from table.
pub(crate) fn xlat(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: XLAT", cpu.current_address); }
  //AL = [DS:BX + unsigned AL]
  let offset = cpu.regs.get_byte(&register::Byte::AL) as u16;
  cpu.memory.current_segment = memory::Segment::DS;
  let value = cpu.memory.get_byte(cpu.regs.bx + offset);
  cpu.regs.set_byte(&register::Byte::AL, value);
  11
}


pub(crate) fn in_al_byte(cpu: &mut CPU, port: operand::Byte, cpu_actor: &Actor<CPU>) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: IN AL, {}", cpu.current_address, port.label()); }
  let port_val = cpu.read_byte(&port) as u16;
  let ret = ret_some_to!([cpu_actor], set_al() as (u8));
  call!([cpu.board], in_byte(port_val, ret));
  14
}
pub(crate) fn in_al_word(cpu: &mut CPU, cpu_actor: &Actor<CPU>) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: IN AL, DX", cpu.current_address); }
  let port_val = cpu.regs.get_word(&register::Word::DX);
  let ret = ret_some_to!([cpu_actor], set_al() as (u8));
  call!([cpu.board], in_byte(port_val, ret));
  12
}

pub(crate) fn in_ax_byte(cpu: &mut CPU, port: operand::Byte, cpu_actor: &Actor<CPU>) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: IN AX, {}", cpu.current_address, port.label()); }
  let port_val = cpu.read_byte(&port) as u16;
  let ret = ret_some_to!([cpu_actor], set_ax() as (u16));
  call!([cpu.board], in_word(port_val, ret));
  14
}
pub(crate) fn in_ax_word(cpu: &mut CPU, cpu_actor: &Actor<CPU>) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: IN AX, DX", cpu.current_address); }
  let port_val = cpu.regs.get_word(&register::Word::DX);
  let ret = ret_some_to!([cpu_actor], set_ax() as (u16));
  call!([cpu.board], in_word(port_val, ret));
  12
}


use simplelog::*;
use std::io::prelude::*;
use std::fs::File;

pub(crate) fn out_al_byte(cpu: &mut CPU, port: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: OUT {}, AL", cpu.current_address, port.label()); }
  let port_val = cpu.read_byte(&port) as u16;
  let value = cpu.regs.get_byte(&register::Byte::AL);
  call!([cpu.board], out_byte(port_val, value));
  14
}
pub(crate) fn out_ax_byte(cpu: &mut CPU, port: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: OUT {}, AX", cpu.current_address, port.label()); }
  let port_val = cpu.read_byte(&port) as u16;
  let value = cpu.regs.get_word(&register::Word::AX);
  call!([cpu.board], out_word(port_val, value));
  14
}

pub(crate) fn out_al_word(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: OUT DX, AL", cpu.current_address); }
  let port_val = cpu.regs.get_word(&register::Word::DX);
  let value = cpu.regs.get_byte(&register::Byte::AL);
  call!([cpu.board], out_byte(port_val, value));

/*
  if !cpu.logging && port_val == 0x321 {
    
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Trace, Config::default(), File::create("trace.log").unwrap()),
    ]).unwrap();
    cpu.logging = true;
  }*/

  12
}
pub(crate) fn out_ax_word(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: OUT DX, AX", cpu.current_address); }
  let port_val = cpu.regs.get_word(&register::Word::DX);
  let value = cpu.regs.get_word(&register::Word::AX);
  call!([cpu.board], out_word(port_val, value));
  12
}
