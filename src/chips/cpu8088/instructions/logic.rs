use super::super::CPU;
use super::super::definitions::operand;

use log::Level::Trace;
use log::{trace, log_enabled};

pub(crate) fn and_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: AND {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let result = cpu.read_byte(&set_op) & cpu.read_byte(&get_op);
  cpu.flags.test_and_or_xor_byte(result);
  cpu.write_byte(&set_op, result);
  set_op.get_cycles_slow(&get_op)
}
pub(crate) fn and_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: AND {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let result = cpu.read_word(&set_op) & cpu.read_word(&get_op);
  cpu.flags.test_and_or_xor_word(result);
  cpu.write_word(&set_op, result);
  set_op.get_cycles_slow(&get_op)
}

pub(crate) fn or_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: OR {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let result = cpu.read_byte(&set_op) | cpu.read_byte(&get_op);
  cpu.flags.test_and_or_xor_byte(result);
  cpu.write_byte(&set_op, result);
  set_op.get_cycles_slow(&get_op)
}
pub(crate) fn or_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: OR {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let result = cpu.read_word(&set_op) | cpu.read_word(&get_op);
  cpu.flags.test_and_or_xor_word(result);
  cpu.write_word(&set_op, result);
  set_op.get_cycles_slow(&get_op)
}

pub(crate) fn xor_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: XOR {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let result = cpu.read_byte(&set_op) ^ cpu.read_byte(&get_op);
  cpu.flags.test_and_or_xor_byte(result);
  cpu.write_byte(&set_op, result);
  set_op.get_cycles_slow(&get_op)
}
pub(crate) fn xor_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: XOR {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let result = cpu.read_word(&set_op) ^ cpu.read_word(&get_op);
  cpu.flags.test_and_or_xor_word(result);
  cpu.write_word(&set_op, result);
  set_op.get_cycles_slow(&get_op)
}

pub(crate) fn test_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: TEST {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let result = cpu.read_byte(&set_op) & cpu.read_byte(&get_op);
  cpu.flags.test_and_or_xor_byte(result);
  set_op.get_cycles_fast(&get_op)
}
pub(crate) fn test_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: TEST {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let result = cpu.read_word(&set_op) & cpu.read_word(&get_op);
  cpu.flags.test_and_or_xor_word(result);
  set_op.get_cycles_fast(&get_op)
}

pub(crate) fn not_byte(cpu: &mut CPU, op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: NOT {}", cpu.current_address, op.label()); }
  let result = !cpu.read_byte(&op);
  cpu.write_byte(&op, result);
  op.get_cycles()
}
pub(crate) fn not_word(cpu: &mut CPU, op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: NOT {}", cpu.current_address, op.label()); }
  let result = !cpu.read_word(&op);
  cpu.write_word(&op, result);
  op.get_cycles()
}
