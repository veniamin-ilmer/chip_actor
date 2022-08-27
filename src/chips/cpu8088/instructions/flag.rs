use super::super::CPU;
use super::super::definitions::register;
use super::super::definitions::operand;
use super::super::definitions::memory;

use log::Level::Trace;
use log::{trace, log_enabled};

pub(crate) fn cmc(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: CMC", cpu.current_address); }
  cpu.flags.carry = !cpu.flags.carry;
  2
}
pub(crate) fn clc(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: CLC", cpu.current_address); }
  cpu.flags.carry = false;
  2
}
pub(crate) fn stc(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: STC", cpu.current_address); }
  cpu.flags.carry = true;
  2
}

pub(crate) fn cli(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: CLI", cpu.current_address); }
  cpu.flags.interrupt = false;
  2
}
pub(crate) fn sti(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: STI", cpu.current_address); }
  cpu.flags.interrupt = true;
  2
}

pub(crate) fn cld(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: CLD", cpu.current_address); }
  cpu.flags.direction = false;
  2
}
pub(crate) fn std(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: STD", cpu.current_address); }
  cpu.flags.direction = true;
  2
}

pub(crate) fn generic_push(cpu: &mut CPU, value: u16) {
  cpu.memory.current_segment = memory::Segment::SS;
  cpu.regs.sp -= 2;
  cpu.memory.set_word(cpu.regs.sp, value);
}
pub(crate) fn push(cpu: &mut CPU, op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: PUSH {}", cpu.current_address, op.label()); }
  let value = cpu.read_word(&op);
  generic_push(cpu, value);
  match op {
    operand::Word::Seg(_) => 14,
    operand::Word::Reg(_) => 15,
    operand::Word::Mem{cycles, ..} => 24 + cycles,
    operand::Word::Imm(_) => unreachable!("Attempted to push an immediate.."),
  }
}
pub(crate) fn pushf(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: PUSHF", cpu.current_address); }
  let value = cpu.flags.get_bits_word();
  generic_push(cpu, value);
  14
}

pub(crate) fn generic_pop(cpu: &mut CPU) -> u16 {
  cpu.memory.current_segment = memory::Segment::SS;
  let value = cpu.memory.get_word(cpu.regs.sp);
  cpu.regs.sp += 2;
  value
}
pub(crate) fn pop(cpu: &mut CPU, op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: POP {}", cpu.current_address, op.label()); }
  let value = generic_pop(cpu);
  cpu.write_word(&op, value);
  match op {
    operand::Word::Seg(_) => 12,
    operand::Word::Reg(_) => 12,
    operand::Word::Mem{cycles, ..} => 25 + cycles,
    operand::Word::Imm(_) => unreachable!("Attempted to push an immediate.."),
  }
}
pub(crate) fn popf(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: POPF", cpu.current_address); }
  let value = generic_pop(cpu);
  cpu.flags.set_bits_word(value);
  12
}

pub(crate) fn lahf(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: LAHF", cpu.current_address); }
  let value = cpu.flags.get_bits_byte();
  cpu.regs.set_byte(&register::Byte::AH, value);
  4
}

pub(crate) fn sahf(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SAHF", cpu.current_address); }
  let value = cpu.regs.get_byte(&register::Byte::AH);
  cpu.flags.set_bits_byte(value);
  4
}