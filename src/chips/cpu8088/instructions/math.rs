use super::super::CPU;
use super::super::definitions::operand;
use super::super::definitions::register;

use log::Level::Trace;
use log::{trace, log_enabled};

pub(crate) fn inc_byte(cpu: &mut CPU, op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: INC {}", cpu.current_address, op.label()); }
  let value = cpu.read_byte(&op);
  let result = cpu.flags.inc_byte(value);
  cpu.write_byte(&op, result);
  op.get_cycles()
}
pub(crate) fn inc_word(cpu: &mut CPU, op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: INC {}", cpu.current_address, op.label()); }
  let value = cpu.read_word(&op);
  let result = cpu.flags.inc_word(value);
  cpu.write_word(&op, result);
  op.get_cycles()
}

pub(crate) fn dec_byte(cpu: &mut CPU, op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: DEC {}", cpu.current_address, op.label()); }
  let value = cpu.read_byte(&op);
  let result = cpu.flags.dec_byte(value);
  cpu.write_byte(&op, result);
  op.get_cycles()
}
pub(crate) fn dec_word(cpu: &mut CPU, op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: DEC {}", cpu.current_address, op.label()); }
  let value = cpu.read_word(&op);
  let result = cpu.flags.dec_word(value);
  cpu.write_word(&op, result);
  op.get_cycles()
}

pub(crate) fn add_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: ADD {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let result = cpu.flags.add_byte(set_val, get_val);
  cpu.write_byte(&set_op, result);
  set_op.get_cycles_slow(&get_op)
}
pub(crate) fn add_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: ADD {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_word(&get_op));
  let result = cpu.flags.add_word(set_val, get_val);
  cpu.write_word(&set_op, result);
  set_op.get_cycles_slow(&get_op)
}

pub(crate) fn adc_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: ADC {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let result = cpu.flags.adc_byte(set_val, get_val);
  cpu.write_byte(&set_op, result);
  set_op.get_cycles_slow(&get_op)
}
pub(crate) fn adc_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: ADC {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_word(&get_op));
  let result = cpu.flags.adc_word(set_val, get_val);
  cpu.write_word(&set_op, result);
  set_op.get_cycles_slow(&get_op)
}

pub(crate) fn sub_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SUB {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let result = cpu.flags.cmp_sub_byte(set_val, get_val);
  cpu.write_byte(&set_op, result);
  set_op.get_cycles_slow(&get_op)
}
pub(crate) fn sub_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SUB {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_word(&get_op));
  let result = cpu.flags.cmp_sub_word(set_val, get_val);
  cpu.write_word(&set_op, result);
  set_op.get_cycles_slow(&get_op)
}

pub(crate) fn cmp_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: CMP {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  cpu.flags.cmp_sub_byte(set_val, get_val);
  set_op.get_cycles_fast(&get_op)
}
pub(crate) fn cmp_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: CMP {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_word(&get_op));
  cpu.flags.cmp_sub_word(set_val, get_val);
  set_op.get_cycles_fast(&get_op)
}

pub(crate) fn sbb_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SBB {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let result = cpu.flags.sbb_byte(set_val, get_val);
  cpu.write_byte(&set_op, result);
  set_op.get_cycles_slow(&get_op)
}
pub(crate) fn sbb_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SBB {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_word(&get_op));
  let result = cpu.flags.sbb_word(set_val, get_val);
  cpu.write_word(&set_op, result);
  set_op.get_cycles_slow(&get_op)
}


pub(crate) fn neg_byte(cpu: &mut CPU, op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: NEG {}", cpu.current_address, op.label()); }
  let value = !cpu.read_byte(&op);
  let result = cpu.flags.inc_byte(value);
  cpu.write_byte(&op, result);
  op.get_cycles()
}
pub(crate) fn neg_word(cpu: &mut CPU, op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: NEG {}", cpu.current_address, op.label()); }
  let value = !cpu.read_word(&op);
  let result = cpu.flags.inc_word(value);
  cpu.write_word(&op, result);
  op.get_cycles()
}

//Unsigned multiply
pub(crate) fn mul_byte(cpu: &mut CPU, op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: MUL {}", cpu.current_address, op.label()); }
  let al = cpu.regs.get_byte(&register::Byte::AL) as u16;
  let value = cpu.read_byte(&op) as u16;
  let result = al * value;
  if result & 0b1000_0000 != 0 {
    cpu.flags.carry = true;
    cpu.flags.overflow = true;
  } else {
    cpu.flags.carry = false;
    cpu.flags.overflow = false;
  }
  cpu.regs.set_word(&register::Word::AX, result);
  120
}
pub(crate) fn mul_word(cpu: &mut CPU, op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: MUL {}", cpu.current_address, op.label()); }
  let ax = cpu.regs.get_word(&register::Word::AX) as u32;
  let value = cpu.read_word(&op) as u32;
  let result = ax * value;
  if result & 0b1000_0000_0000_0000 != 0 {
    cpu.flags.carry = true;
    cpu.flags.overflow = true;
  } else {
    cpu.flags.carry = false;
    cpu.flags.overflow = false;
  }
  let [al, ah, dl, dh] = result.to_le_bytes();
  let ax = u16::from_le_bytes([al, ah]);
  let dx = u16::from_le_bytes([dl, dh]);
  cpu.regs.set_word(&register::Word::AX, ax);
  cpu.regs.set_word(&register::Word::DX, dx);
  120
}

//Signed multiply
pub(crate) fn imul_byte(cpu: &mut CPU, op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: MUL {}", cpu.current_address, op.label()); }
  let al = (cpu.regs.get_byte(&register::Byte::AL) as i8) as i16;
  let value = (cpu.read_byte(&op) as i8) as i16;
  let result = (al * value) as u16;
  cpu.flags.carry = false;  //TODO - confirm this?
  cpu.flags.overflow = false;
  cpu.regs.set_word(&register::Word::AX, result);
  120
}
pub(crate) fn imul_word(cpu: &mut CPU, op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: MUL {}", cpu.current_address, op.label()); }
  let ax = (cpu.regs.get_word(&register::Word::AX) as i16) as i32;
  let value = (cpu.read_word(&op) as i16) as i32;
  let result = (ax * value) as u32;
  cpu.flags.carry = false;  //TODO - confirm this?
  cpu.flags.overflow = false;
  let [al, ah, dl, dh] = result.to_le_bytes();
  let ax = u16::from_le_bytes([al, ah]);
  let dx = u16::from_le_bytes([dl, dh]);
  cpu.regs.set_word(&register::Word::AX, ax);
  cpu.regs.set_word(&register::Word::DX, dx);
  120
}

//Unsigned divide
pub(crate) fn div_byte(cpu: &mut CPU, op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: DIV {}", cpu.current_address, op.label()); }
  let ax = cpu.regs.get_word(&register::Word::AX);
  let value = cpu.read_byte(&op) as u16;
  cpu.regs.set_byte(&register::Byte::AL, (ax / value) as u8);
  cpu.regs.set_byte(&register::Byte::AH, (ax % value) as u8);
  120
}
pub(crate) fn div_word(cpu: &mut CPU, op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: DIV {}", cpu.current_address, op.label()); }
  let dx = cpu.regs.get_word(&register::Word::DX);
  let ax = cpu.regs.get_word(&register::Word::AX);
  let [al, ah] = ax.to_le_bytes();
  let [dl, dh] = dx.to_le_bytes();
  let full_number = u32::from_le_bytes([al, ah, dl, dh]);
  let value = cpu.read_word(&op) as u32;
  cpu.regs.set_word(&register::Word::AX, (full_number / value) as u16);
  cpu.regs.set_word(&register::Word::DX, (full_number % value) as u16);
  120
}

//Signed divide
pub(crate) fn idiv_byte(cpu: &mut CPU, op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: IDIV {}", cpu.current_address, op.label()); }
  let ax = cpu.regs.get_word(&register::Word::AX) as i16;
  let value = (cpu.read_byte(&op) as i8) as i16;
  cpu.regs.set_byte(&register::Byte::AL, (ax / value) as u8);
  cpu.regs.set_byte(&register::Byte::AH, (ax % value) as u8);
  120
}
pub(crate) fn idiv_word(cpu: &mut CPU, op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: IDIV {}", cpu.current_address, op.label()); }
  let dx = cpu.regs.get_word(&register::Word::DX);
  let ax = cpu.regs.get_word(&register::Word::AX);
  let [al, ah] = ax.to_le_bytes();
  let [dl, dh] = dx.to_le_bytes();
  let full_number = u32::from_le_bytes([al, ah, dl, dh]) as i32;
  let value = (cpu.read_word(&op) as i16) as i32;
  cpu.regs.set_word(&register::Word::AX, (full_number / value) as u16);
  cpu.regs.set_word(&register::Word::DX, (full_number % value) as u16);
  120
}

//Convert Byte to Word
pub(crate) fn cbw(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: CBW", cpu.current_address); }
  if (cpu.regs.get_byte(&register::Byte::AL) as i8) < 0 {
    cpu.regs.set_byte(&register::Byte::AH, 0xFF);
  } else {
    cpu.regs.set_byte(&register::Byte::AH, 0x0);
  }
  2
}

//Covert Word to Double
pub(crate) fn cwd(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: CWD", cpu.current_address); }
  if (cpu.regs.get_word(&register::Word::AX) as i16) < 0 {
    cpu.regs.set_word(&register::Word::DX, 0xFF);
  } else {
    cpu.regs.set_word(&register::Word::DX, 0x0);
  }
  5
}
