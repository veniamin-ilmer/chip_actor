use stakker::*;

use super::super::CPU;
use super::super::definitions::operand;
use super::flag;

use super::lookup;

use log::Level::{Debug, Trace};
use log::{debug, trace, log_enabled};


pub(crate) fn jmp_word(cpu: &mut CPU, op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: JMP {}", cpu.current_address, op.label()); }
  let value = cpu.read_word(&op);
  cpu.memory.ip = value;
  15
}

pub(crate) fn jmp_addr(cpu: &mut CPU, segment: operand::Word, offset: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: JMP {}:{}", cpu.current_address, segment.label(), offset.label()); }
  let (seg, off) = (cpu.read_word(&segment), cpu.read_word(&offset));
  cpu.memory.cs = seg;
  cpu.memory.ip = off;
  15
}

pub(crate) fn jmp_relative(cpu: &mut CPU, relative_offset: i8, condition: bool) -> usize {
  let offset = (cpu.memory.ip as i16) + relative_offset as i16;
  if condition {
    cpu.memory.ip = offset as u16;
    16
  } else {
    4
  }
}

pub(crate) fn jmp_relative_word(cpu: &mut CPU, relative_offset: i16) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: JMP +{:X}", cpu.current_address, relative_offset); }
  let offset = (cpu.memory.ip as i16) + relative_offset;
  cpu.memory.ip = offset as u16;
  15
}

pub(crate) fn jmp_far(cpu: &mut CPU, op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: JMP FAR {}", cpu.current_address, op.label()); }
  match op {
    operand::Word::Mem{addr, cycles, ..} => {
      let offset = cpu.memory.get_word(addr);
      let segment = cpu.memory.get_word(addr + 2);
      cpu.memory.cs = segment;
      cpu.memory.ip = offset;
      24 + cycles
    },
    _ => {//Tried to jump to a far location without providing a segment. Revert to jumping to just a word.
      panic!("{:05X}: Incorrect Jump Far.", cpu.current_address);
    },
  }
}

pub(crate) fn call_word(cpu: &mut CPU, offset: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: CALL {}", cpu.current_address, offset.label()); }
  let off_val = cpu.read_word(&offset);
  flag::generic_push(cpu, cpu.memory.ip);
  cpu.memory.ip = off_val;
  match offset {
    operand::Word::Mem{cycles, ..} => 29+cycles,
    _ => 21,
  }
}

pub(crate) fn call_relative_word(cpu: &mut CPU, offset: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: CALL +{}", cpu.current_address, offset.label()); }
  let relative_offset = cpu.read_word(&offset) as i16;
  let offset_num = (cpu.memory.ip as i16) + relative_offset;
  flag::generic_push(cpu, cpu.memory.ip);
  cpu.memory.ip = offset_num as u16;
  match offset {
    operand::Word::Mem{cycles, ..} => 29+cycles,
    _ => 21,
  }
}

pub(crate) fn call_addr(cpu: &mut CPU, segment: operand::Word, offset: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: CALL {}:{}", cpu.current_address, segment.label(), offset.label()); }
  let (seg, off) = (cpu.read_word(&segment), cpu.read_word(&offset));
  flag::generic_push(cpu, cpu.memory.cs);
  flag::generic_push(cpu, cpu.memory.ip);
  cpu.memory.cs = seg;
  cpu.memory.ip = off;
  36
}

pub(crate) fn call_far(cpu: &mut CPU, op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: CALL FAR {}", cpu.current_address, op.label()); }
  match op {
    operand::Word::Mem{addr, cycles, ..} => {
      let offset = cpu.memory.get_word(addr);
      let segment = cpu.memory.get_word(addr + 2);
      flag::generic_push(cpu, cpu.memory.cs);
      flag::generic_push(cpu, cpu.memory.ip);
      cpu.memory.cs = segment;
      cpu.memory.ip = offset;
      53 + cycles
    },
    _ => {//Tried to jump to a far location without providing a segment.
      panic!("{:05X}: Incorrect Call Far.", cpu.current_address);
    },
  }
}

pub(crate) fn ret(cpu: &mut CPU, add_sp: Option<u16>) -> usize {
  cpu.memory.ip = flag::generic_pop(cpu);
  if let Some(num) = add_sp {
    if log_enabled!(Trace) { trace!("{:05X}: RET {:X}", cpu.current_address, num); }
    cpu.regs.sp += num;
    24
  } else {
    if log_enabled!(Trace) { trace!("{:05X}: RET", cpu.current_address); }
    20
  }
}

pub(crate) fn retf(cpu: &mut CPU, add_sp: Option<u16>) -> usize {
  cpu.memory.ip = flag::generic_pop(cpu);
  cpu.memory.cs = flag::generic_pop(cpu);
  if let Some(num) = add_sp {
    if log_enabled!(Trace) { trace!("{:05X}: RETF {:X}", cpu.current_address, num); }
    cpu.regs.sp += num;
    33
  } else {
    if log_enabled!(Trace) { trace!("{:05X}: RETF", cpu.current_address); }
    34
  }
}

fn _int(cpu: &mut CPU, index: u8) -> usize {
  flag::generic_push(cpu, cpu.flags.get_bits_word());
  cpu.flags.interrupt = false;  //Interrupts are not allowed while inside of an interrupt.
  flag::generic_push(cpu, cpu.memory.cs);
  flag::generic_push(cpu, cpu.memory.ip);
  if log_enabled!(Debug) { debug!("Interrupt {:X}", index); }
  cpu.print_registers();
  cpu.memory.ip = cpu.memory.get_word_at_addr(index as usize * 4);
  cpu.memory.cs = cpu.memory.get_word_at_addr(index as usize * 4 + 2);
  72
}

pub(crate) fn hardware_int(cpu: &mut CPU, index: u8) -> usize {
  if log_enabled!(Trace) { trace!("HARDWARE INT {:X}", index); }
  _int(cpu, index)
}

pub(crate) fn int(cpu: &mut CPU, index: u8) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: INT {:X}", cpu.current_address, index); }
  _int(cpu, index)
}

pub(crate) fn into(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: INTO", cpu.current_address); }
  if cpu.flags.overflow {
    _int(cpu, 3)
  } else {
    4
  }
}

pub(crate) fn iret(cpu: &mut CPU) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: IRET", cpu.current_address); }
  cpu.memory.ip = flag::generic_pop(cpu);
  cpu.memory.cs = flag::generic_pop(cpu);
  let flag_word = flag::generic_pop(cpu);
  cpu.flags.set_bits_word(flag_word);
  44
}

pub(crate) fn loop_relative(cpu: &mut CPU, relative_offset: i8, condition: bool) -> usize {
  cpu.regs.cx = cpu.regs.cx.wrapping_sub(1);
  if cpu.regs.cx != 0 {
    jmp_relative(cpu, relative_offset, condition) + 2
  } else {
    6
  }
}

pub(crate) fn rep(cpu: &mut CPU, zero: bool, cpu_actor: &Actor<CPU>) -> usize {
  if log_enabled!(Trace) {
    if zero {
      trace!("{:05X}: REPZ", cpu.current_address);
    } else {
      trace!("{:05X}: REPNZ", cpu.current_address);
    }
  }
  if cpu.regs.cx != 0 {
    let prev_ip = cpu.memory.ip;
    lookup::run_next_instruction(cpu, cpu_actor);
    cpu.regs.cx -= 1;
    if zero == cpu.flags.zero {
      cpu.memory.ip = prev_ip - 1;  //Next time we run an instruction should be back at this rep.
    }
  }
  if cpu.regs.cx == 0 {
    cpu.flags.zero = true;
  }
  0
}
