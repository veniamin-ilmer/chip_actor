use super::super::CPU;
use super::super::definitions::register;

use log::Level::Trace;
use log::{trace, log_enabled};

//ASCII adjust After Addition
pub(crate) fn aaa(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: AAA", cpu.current_address); }
  let mut al = cpu.regs.get_byte(&register::Byte::AL);
  let mut ah = cpu.regs.get_byte(&register::Byte::AH);
  if (al & 0xF) > 9 || cpu.flags.adjust {
    al += 6;
    ah += 1;
    cpu.regs.set_byte(&register::Byte::AH, ah);
    cpu.flags.adjust = true;
    cpu.flags.carry = true;
  } else {
    cpu.flags.adjust = false;
    cpu.flags.carry = false;
  }
  cpu.regs.set_byte(&register::Byte::AL, al & 0xF);
  8
}

//ASCII adjust After Subtraction
pub(crate) fn aas(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: AAS", cpu.current_address); }
  let mut al = cpu.regs.get_byte(&register::Byte::AL);
  let mut ah = cpu.regs.get_byte(&register::Byte::AH);
  if (al & 0xF) > 9 || cpu.flags.adjust {
    al -= 6;
    ah -= 1;
    cpu.regs.set_byte(&register::Byte::AH, ah);
    cpu.flags.adjust = true;
    cpu.flags.carry = true;
  } else {
    cpu.flags.adjust = false;
    cpu.flags.carry = false;
  }
  cpu.regs.set_byte(&register::Byte::AL, al & 0xF);
  8
}

//ASCII adjust After Multiplication
pub(crate) fn aam(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: AAM", cpu.current_address); }
  let al = cpu.regs.get_byte(&register::Byte::AL);
  cpu.regs.set_byte(&register::Byte::AH, al / 10);
  cpu.regs.set_byte(&register::Byte::AL, al % 10);
  cpu.flags.parity_zero_sign_byte(al / 10);  //TODO: Perhaps both AL and AH need to be included here?
  83
}

//ASCII adjust before? Division
pub(crate) fn aad(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: AAD", cpu.current_address); }
  let al = cpu.regs.get_byte(&register::Byte::AL);
  let ah = cpu.regs.get_byte(&register::Byte::AH);
  let result = (ah * 10) + al;
  cpu.regs.set_byte(&register::Byte::AL, result);
  cpu.regs.set_byte(&register::Byte::AH, 0);
  cpu.flags.parity_zero_sign_byte(result);
  60
}

//Decimal adjust After Addition
pub(crate) fn daa(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: DAA", cpu.current_address); }
  let mut al = cpu.regs.get_byte(&register::Byte::AL);
  if (al & 0xF) > 9 || cpu.flags.adjust {
    al += 6;
    cpu.flags.adjust = true;
  } else {
    cpu.flags.adjust = false;
  }
  if al > 0x9F || cpu.flags.carry {
    al = al.wrapping_add(0x60);
    cpu.flags.carry = true;
  } else {
    cpu.flags.carry = false;
  }
  cpu.regs.set_byte(&register::Byte::AL, al);
  4
}

//Decimal adjust After Subtraction
pub(crate) fn das(cpu: &mut CPU)-> usize {
  if log_enabled!(Trace) { trace!("{:05X}: DAS", cpu.current_address); }
  let mut al = cpu.regs.get_byte(&register::Byte::AL);
  if (al & 0xF) > 9 || cpu.flags.adjust {
    al -= 6;
    cpu.flags.adjust = true;
  } else {
    cpu.flags.adjust = false;
  }
  if al > 0x9F || cpu.flags.carry {
    al = al.wrapping_sub(0x60);
    cpu.flags.carry = true;
  } else {
    cpu.flags.carry = false;
  }
  cpu.regs.set_byte(&register::Byte::AL, al);
  4
}
