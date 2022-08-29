use super::super::CPU;

use super::super::definitions::operand;

use log::Level::Trace;
use log::{trace, log_enabled};

pub(crate) fn shr_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SHR {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let result = cpu.flags.shr_ror_rcr_byte(set_val, get_val);
  cpu.write_byte(&set_op, result);
  set_op.get_rotate_cycles(&get_op, get_val)
}
pub(crate) fn shr_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SHR {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_byte(&get_op));
  let result = cpu.flags.shr_ror_rcr_word(set_val, get_val);
  cpu.write_word(&set_op, result);
  set_op.get_rotate_cycles(&get_op, get_val)
}

pub(crate) fn sar_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SAR {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let mut result = cpu.flags.shr_ror_rcr_byte(set_val, get_val);
  if (set_val as i8) < 0 {
    result |= 0b1000_0000;
  }
  cpu.write_byte(&set_op, result);
  set_op.get_rotate_cycles(&get_op, get_val)
}
pub(crate) fn sar_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SAR {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_byte(&get_op));
  let mut result = cpu.flags.shr_ror_rcr_word(set_val, get_val);
  if (set_val as i8) < 0 {
    result |= 0b1000_0000;
  }
  cpu.write_word(&set_op, result);
  set_op.get_rotate_cycles(&get_op, get_val)
}

pub(crate) fn ror_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SHR {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let mut result = cpu.flags.shr_ror_rcr_byte(set_val, get_val);
  if cpu.flags.carry {
    result |= 0b1000_0000;
  }
  cpu.write_byte(&set_op, result);
  set_op.get_rotate_cycles(&get_op, get_val)
}
pub(crate) fn ror_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SHR {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_byte(&get_op));
  let mut result = cpu.flags.shr_ror_rcr_word(set_val, get_val);
  if cpu.flags.carry {
    result |= 0b1000_0000;
  }
  cpu.write_word(&set_op, result);
  set_op.get_rotate_cycles(&get_op, get_val)
}

pub(crate) fn rcr_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SHR {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let old_carry = cpu.flags.carry;
  let mut result = cpu.flags.shr_ror_rcr_byte(set_val, get_val);
  if old_carry {
    result |= 0b1000_0000;
  }
  cpu.write_byte(&set_op, result);
  set_op.get_rotate_cycles(&get_op, get_val)
}
pub(crate) fn rcr_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SHR {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_byte(&get_op));
  let old_carry = cpu.flags.carry;
  let mut result = cpu.flags.shr_ror_rcr_word(set_val, get_val);
  if old_carry {
    result |= 0b1000_0000;
  }
  cpu.write_word(&set_op, result);
  set_op.get_rotate_cycles(&get_op, get_val)
}

pub(crate) fn shl_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SHL {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let result = cpu.flags.shl_rol_rcl_byte(set_val, get_val);
  cpu.write_byte(&set_op, result);
  set_op.get_rotate_cycles(&get_op, get_val)
}
pub(crate) fn shl_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: SHL {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_byte(&get_op));
  let result = cpu.flags.shl_rol_rcl_word(set_val, get_val);
  cpu.write_word(&set_op, result);
  set_op.get_rotate_cycles(&get_op, get_val)
}

pub(crate) fn rol_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: ROL {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let mut result = cpu.flags.shl_rol_rcl_byte(set_val, get_val);
  if cpu.flags.carry {
    result |= 1;
  }
  cpu.write_byte(&set_op, result);
  set_op.get_rotate_cycles(&get_op, get_val)
}
pub(crate) fn rol_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: ROL {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_byte(&get_op));
  let mut result = cpu.flags.shl_rol_rcl_word(set_val, get_val);
  if cpu.flags.carry {
    result |= 1;
  }
  cpu.write_word(&set_op, result);
  set_op.get_rotate_cycles(&get_op, get_val)
}

pub(crate) fn rcl_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: RCL {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let old_carry = cpu.flags.carry;
  let mut result = cpu.flags.shl_rol_rcl_byte(set_val, get_val);
  if old_carry {
    result |= 1;
  }
  cpu.write_byte(&set_op, result);
  set_op.get_rotate_cycles(&get_op, get_val)
}
pub(crate) fn rcl_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: RCL {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_byte(&get_op));
  let old_carry = cpu.flags.carry;
  let mut result = cpu.flags.shl_rol_rcl_word(set_val, get_val);
  if old_carry {
    result |= 1;
  }
  cpu.write_word(&set_op, result);
  set_op.get_rotate_cycles(&get_op, get_val)
}
