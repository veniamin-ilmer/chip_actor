use super::super::CPU;
use super::super::definitions::memory;
use super::super::definitions::register;

use log::Level::Trace;
use log::{trace, log_enabled};

fn move_si(cpu: &mut CPU, amount: u16) {
  if !cpu.flags.direction {
    cpu.regs.si = cpu.regs.si.wrapping_add(amount);
  } else {
    cpu.regs.si = cpu.regs.si.wrapping_sub(amount);
  }
}
fn move_di(cpu: &mut CPU, amount: u16) {
  if !cpu.flags.direction {
    cpu.regs.di = cpu.regs.di.wrapping_add(amount);
  } else {
    cpu.regs.di = cpu.regs.di.wrapping_sub(amount);
  }
}

//Move String Byte
pub(crate) fn movsb(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: MOVSB", cpu.current_address); }
  //[ES:DI] = [DS:SI]
  cpu.memory.current_segment = memory::Segment::DS;
  let value = cpu.memory.get_byte(cpu.regs.si);
  cpu.memory.current_segment = memory::Segment::ES;
  cpu.memory.set_byte(cpu.regs.di, value);

  move_si(cpu, 1);
  move_di(cpu, 1);
  18
}
//Move String Word
pub(crate) fn movsw(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: MOVSW", cpu.current_address); }
  //[ES:DI] = [DS:SI]
  cpu.memory.current_segment = memory::Segment::DS;
  let value = cpu.memory.get_word(cpu.regs.si);
  cpu.memory.current_segment = memory::Segment::ES;
  cpu.memory.set_word(cpu.regs.di, value);
  
  move_si(cpu, 2);
  move_di(cpu, 2);
  26
}

//Compare String Byte
pub(crate) fn cmpsb(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: CMPSB", cpu.current_address); }
  //[DS:SI] - [ES:DI]
  cpu.memory.current_segment = memory::Segment::DS;
  let set_val = cpu.memory.get_byte(cpu.regs.si);
  cpu.memory.current_segment = memory::Segment::ES;
  let get_val = cpu.memory.get_byte(cpu.regs.di);
  cpu.flags.cmp_sub_byte(set_val, get_val);
  
  move_si(cpu, 1);
  move_di(cpu, 1);
  30
}
//Compare String Word
pub(crate) fn cmpsw(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: CMPSW", cpu.current_address); }
  //[DS:SI] - [ES:DI]
  cpu.memory.current_segment = memory::Segment::DS;
  let set_val = cpu.memory.get_word(cpu.regs.si);
  cpu.memory.current_segment = memory::Segment::ES;
  let get_val = cpu.memory.get_word(cpu.regs.di);
  cpu.flags.cmp_sub_word(set_val, get_val);
  
  move_si(cpu, 2);
  move_di(cpu, 2);
  30
}

pub(crate) fn lodsb(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: LODSB", cpu.current_address); }
  //AL = [DS:SI]
  cpu.memory.current_segment = memory::Segment::DS;
  let value = cpu.memory.get_byte(cpu.regs.si);
  cpu.regs.set_byte(&register::Byte::AL, value);

  move_si(cpu, 1);
  16
}
pub(crate) fn lodsw(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: LODSW", cpu.current_address); }
  //AX = [DS:SI]
  cpu.memory.current_segment = memory::Segment::DS;
  let value = cpu.memory.get_word(cpu.regs.si);
  cpu.regs.set_word(&register::Word::AX, value);

  move_si(cpu, 2);
  16
}

pub(crate) fn stosb(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: STOSB", cpu.current_address); }
  //[ES:DI] = AL
  let value = cpu.regs.get_byte(&register::Byte::AL);
  cpu.memory.current_segment = memory::Segment::ES;
  cpu.memory.set_byte(cpu.regs.di, value);

  move_di(cpu, 1);
  11
}
pub(crate) fn stosw(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: STOSW", cpu.current_address); }
  //[ES:DI] = AX
  let value = cpu.regs.get_word(&register::Word::AX);
  cpu.memory.current_segment = memory::Segment::ES;
  cpu.memory.set_word(cpu.regs.di, value);

  move_di(cpu, 2);
  15
}

pub(crate) fn scasb(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SCASB", cpu.current_address); }
  //AL - [ES:DI]
  let set_val = cpu.regs.get_byte(&register::Byte::AL);
  cpu.memory.current_segment = memory::Segment::ES;
  let get_val = cpu.memory.get_byte(cpu.regs.di);
  cpu.flags.cmp_sub_byte(set_val, get_val);
  
  move_di(cpu, 1);
  19
}
pub(crate) fn scasw(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SCASW", cpu.current_address); }
  //AX - [ES:DI]
  let set_val = cpu.regs.get_word(&register::Word::AX);
  cpu.memory.current_segment = memory::Segment::ES;
  let get_val = cpu.memory.get_word(cpu.regs.di);
  cpu.flags.cmp_sub_word(set_val, get_val);
  
  move_di(cpu, 2);
  19
}