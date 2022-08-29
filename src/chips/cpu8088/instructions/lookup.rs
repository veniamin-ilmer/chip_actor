use stakker::*;

/*
  Sources:
  http://mlsite.net/8086/
  https://yassinebridi.github.io/asm-docs/8086_instruction_set.html
*/

use super::super::definitions::memory;
use super::super::definitions::register;
use super::super::definitions::operand;

use log::Level::Trace;
use log::{trace, log_enabled};

use super::*;

pub(crate) fn run_next_instruction(cpu: &mut super::super::CPU, cpu_actor: &Actor<super::super::CPU>) -> usize {
  
  let mut next_segment = memory::Segment::DS; //This is for "ES:, CS:, DS:, SS:" commands for future memory lookup. Current memory lookup is in cpu.memory.current_segment
  
  cpu.current_address = cpu.memory.get_current_address();
  let op0 = cpu.memory.next_byte();

  let cycles = match op0 {
    0x00..=0x05 => {
      match lookup::get_standard_ops(cpu, op0 & 0b111) {
        operand::Pair::Bytes(set_op, get_op) => math::add_byte(cpu, set_op, get_op),
        operand::Pair::Words(set_op, get_op) => math::add_word(cpu, set_op, get_op),
      }
    },
    0x06 => flag::push(cpu, operand::Word::Seg(memory::Segment::ES)),
    0x07 => flag::pop(cpu, operand::Word::Seg(memory::Segment::ES)),
    0x08..=0x0D => {
      match lookup::get_standard_ops(cpu, op0 & 0b111) {
        operand::Pair::Bytes(set_op, get_op) => logic::or_byte(cpu, set_op, get_op),
        operand::Pair::Words(set_op, get_op) => logic::or_word(cpu, set_op, get_op),
      }
    },
    0x0E => flag::push(cpu, operand::Word::Seg(memory::Segment::CS)),
    0x10..=0x15 => {
      match lookup::get_standard_ops(cpu, op0 & 0b111) {
        operand::Pair::Bytes(set_op, get_op) => math::adc_byte(cpu, set_op, get_op),
        operand::Pair::Words(set_op, get_op) => math::adc_word(cpu, set_op, get_op),
      }
    },
    0x16 => flag::push(cpu, operand::Word::Seg(memory::Segment::SS)),
    0x17 => flag::pop(cpu, operand::Word::Seg(memory::Segment::SS)),
    0x18..=0x1D => {
      match lookup::get_standard_ops(cpu, op0 & 0b111) {
        operand::Pair::Bytes(set_op, get_op) => math::sbb_byte(cpu, set_op, get_op),
        operand::Pair::Words(set_op, get_op) => math::sbb_word(cpu, set_op, get_op),
      }
    },
    0x1E => flag::push(cpu, operand::Word::Seg(memory::Segment::DS)),
    0x1F => flag::pop(cpu, operand::Word::Seg(memory::Segment::DS)),
    0x20..=0x25 => {
      match lookup::get_standard_ops(cpu, op0 & 0b111) {
        operand::Pair::Bytes(set_op, get_op) => logic::and_byte(cpu, set_op, get_op),
        operand::Pair::Words(set_op, get_op) => logic::and_word(cpu, set_op, get_op),
      }
    },
    0x26 => {
      if log_enabled!(Trace) { trace!("{:05X}: ES:", cpu.current_address); }
      next_segment = memory::Segment::ES;
      2
    },
    0x27 => bcd::daa(cpu),
    0x28..=0x2D => {
      match lookup::get_standard_ops(cpu, op0 & 0b111) {
        operand::Pair::Bytes(set_op, get_op) => math::sub_byte(cpu, set_op, get_op),
        operand::Pair::Words(set_op, get_op) => math::sub_word(cpu, set_op, get_op),
      }
    },
    0x2E => {
      if log_enabled!(Trace) { trace!("{:05X}: CS:", cpu.current_address); }
      next_segment = memory::Segment::CS;
      2
    },
    0x2F => bcd::das(cpu),
    0x30..=0x35 => {
      match lookup::get_standard_ops(cpu, op0 & 0b111) {
        operand::Pair::Bytes(set_op, get_op) => logic::xor_byte(cpu, set_op, get_op),
        operand::Pair::Words(set_op, get_op) => logic::xor_word(cpu, set_op, get_op),
      }
    },
    0x36 => {
      if log_enabled!(Trace) { trace!("{:05X}: SS:", cpu.current_address); }
      next_segment = memory::Segment::SS;
      2
    },
    0x37 => bcd::aaa(cpu),
    0x38..=0x3D => {
      match lookup::get_standard_ops(cpu, op0 & 0b111) {
        operand::Pair::Bytes(set_op, get_op) => math::cmp_byte(cpu, set_op, get_op),
        operand::Pair::Words(set_op, get_op) => math::cmp_word(cpu, set_op, get_op),
      }
    }
    0x3E => {
      if log_enabled!(Trace) { trace!("{:05X}: CS:", cpu.current_address); }
      next_segment = memory::Segment::CS;
      2
    },
    0x3F => bcd::aas(cpu),
    0x40..=0x47 => math::inc_word(cpu, operand::Word::reg_index(op0 & 7)),
    0x48..=0x4F => math::dec_word(cpu, operand::Word::reg_index(op0 & 7)),
    0x50..=0x57 => flag::push(cpu, operand::Word::reg_index(op0 & 7)),
    0x58..=0x5F => flag::pop(cpu, operand::Word::reg_index(op0 & 7)),
    0x70 => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JO +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, cpu.flags.overflow)
    },
    0x71 => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JNO +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, !cpu.flags.overflow)
    },
    0x72 => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JB +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, cpu.flags.carry)
    },
    0x73 => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JNB +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, !cpu.flags.carry)
    },
    0x74 => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JZ +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, cpu.flags.zero)
    },
    0x75 => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JNZ +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, !cpu.flags.zero)
    },
    0x76 => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JBE +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, cpu.flags.carry || cpu.flags.zero)
    },
    0x77 => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JA +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, !cpu.flags.carry && !cpu.flags.zero)
    },
    0x78 => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JS +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, cpu.flags.sign)
    },
    0x79 => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JNS +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, !cpu.flags.sign)
    },
    0x7A => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JPE +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, cpu.flags.parity)
    },
    0x7B => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JPO +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, !cpu.flags.parity)
    },
    0x7C => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JL +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, cpu.flags.sign != cpu.flags.overflow)
    },
    0x7D => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JGE +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, cpu.flags.sign == cpu.flags.overflow)
    },
    0x7E => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JGE +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, cpu.flags.sign != cpu.flags.overflow || cpu.flags.zero)
    },
    0x7F => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JG +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, cpu.flags.sign == cpu.flags.overflow && !cpu.flags.zero)
    },
    0x80 | 0x82 => {  //Absolutely nothing is different between 0x80 and 0x82. It is a duplicate.
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Byte::extended(&mut cpu.memory, &cpu.regs, op1);
      let get_op = operand::Byte::Imm(cpu.memory.next_byte());
      match (op1 & 0b111000) >> 3 {
        0 => math::add_byte(cpu, set_op, get_op),
        1 => logic::or_byte(cpu, set_op, get_op),
        2 => math::adc_byte(cpu, set_op, get_op),
        3 => math::sbb_byte(cpu, set_op, get_op),
        4 => logic::and_byte(cpu, set_op, get_op),
        5 => math::sub_byte(cpu, set_op, get_op),
        6 => logic::xor_byte(cpu, set_op, get_op),
        7 => math::cmp_byte(cpu, set_op, get_op),
        _ => unreachable!(),
      }
    },
    0x81 | 0x83 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      let get_op = operand::Word::Imm(
        if op0 == 0x81 {
          cpu.memory.next_word()
        } else {
          //This is an interesting opcode that allows us to do an operation between 16 bits and 8 bits. To do this, we treat the 8 bit as signed, then carry the sign to 16 bits.
          ((cpu.memory.next_byte() as i8) as i16) as u16
        }
      );
      match (op1 & 0b111000) >> 3 {
        0 => math::add_word(cpu, set_op, get_op),
        1 => logic::or_word(cpu, set_op, get_op),
        2 => math::adc_word(cpu, set_op, get_op),
        3 => math::sbb_word(cpu, set_op, get_op),
        4 => logic::and_word(cpu, set_op, get_op),
        5 => math::sub_word(cpu, set_op, get_op),
        6 => logic::xor_word(cpu, set_op, get_op),
        7 => math::cmp_word(cpu, set_op, get_op),
        _ => unreachable!(),
      }
    }
    0x84 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Byte::general(op1);
      let get_op = operand::Byte::extended(&mut cpu.memory, &cpu.regs, op1);
      logic::test_byte(cpu, set_op, get_op)
    },
    0x85 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Word::general(op1);
      let get_op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      logic::test_word(cpu, set_op, get_op)
    },
    0x86 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Byte::general(op1);
      let get_op = operand::Byte::extended(&mut cpu.memory, &cpu.regs, op1);
      set::xchg_byte(cpu, set_op, get_op)
    },
    0x87 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Word::general(op1);
      let get_op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      set::xchg_word(cpu, set_op, get_op)
    },
    0x88..=0x8B => {
      match lookup::get_standard_ops(cpu, op0 & 0b111) {
        operand::Pair::Bytes(set_op, get_op) => set::mov_byte(cpu, set_op, get_op),
        operand::Pair::Words(set_op, get_op) => set::mov_word(cpu, set_op, get_op),
      }
    },
    0x8C => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      let get_op = operand::Word::segment(op1);
      set::mov_word(cpu, set_op, get_op)
    },
    0x8D => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Word::general(op1);
      let get_op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      set::lea_word(cpu, set_op, get_op)
    }
    0x8E => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Word::segment(op1);
      let get_op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      set::mov_word(cpu, set_op, get_op)
    },
    0x8F => {
      let op1 = cpu.memory.next_byte();
      let op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      flag::pop(cpu, op)
    }
    0x90 => {
      if log_enabled!(Trace) { trace!("{:05X}: NOP", cpu.current_address); }
      3
    },
    0x91..=0x97 => set::xchg_word(cpu,
                                  operand::Word::reg_index(op0 & 7),
                                  operand::Word::Reg(register::Word::AX)),
    0x98 => math::cbw(cpu),
    0x99 => math::cwd(cpu),
    0x9A => {
      let offset = operand::Word::Imm(cpu.memory.next_word());
      let segment = operand::Word::Imm(cpu.memory.next_word());
      jump::call_addr(cpu, segment, offset)
    },
    0x9B => {
      trace!("{:05X}: WAIT", cpu.current_address);
      panic!("Stop");
    }
    0x9C => flag::pushf(cpu),
    0x9D => flag::popf(cpu),
    0x9E => flag::sahf(cpu),
    0x9F => flag::lahf(cpu),
    0xA0 => {
      let set_op = operand::Byte::Reg(register::Byte::AL);
      let get_op = operand::Byte::address(&mut cpu.memory);
      set::mov_byte(cpu, set_op, get_op)
    },
    0xA1 => {
      let set_op = operand::Word::Reg(register::Word::AX);
      let get_op = operand::Word::address(&mut cpu.memory);
      set::mov_word(cpu, set_op, get_op)
    },
    0xA2 => {
      let set_op = operand::Byte::address(&mut cpu.memory);
      let get_op = operand::Byte::Reg(register::Byte::AL);
      set::mov_byte(cpu, set_op, get_op)
    },
    0xA3 => {
      let set_op = operand::Word::address(&mut cpu.memory);
      let get_op = operand::Word::Reg(register::Word::AX);
      set::mov_word(cpu, set_op, get_op)
    },
    0xA4 => string::movsb(cpu),
    0xA5 => string::movsw(cpu),
    0xA6 => string::cmpsb(cpu),
    0xA7 => string::cmpsw(cpu),
    0xA8 => {
      let set_op = operand::Byte::Reg(register::Byte::AL);
      let get_op = operand::Byte::Imm(cpu.memory.next_byte());
      logic::test_byte(cpu, set_op, get_op)
    },
    0xA9 => {
      let set_op = operand::Word::Reg(register::Word::AX);
      let get_op = operand::Word::Imm(cpu.memory.next_word());
      logic::test_word(cpu, set_op, get_op)
    },
    0xAA => string::stosb(cpu),
    0xAB => string::stosw(cpu),
    0xAC => string::lodsb(cpu),
    0xAD => string::lodsw(cpu),
    0xAE => string::scasb(cpu),
    0xAF => string::scasw(cpu),
    0xB0..=0xB7 => {
      let set_op = operand::Byte::reg_index(op0 & 7); 
      let get_op = operand::Byte::Imm(cpu.memory.next_byte());
      set::mov_byte(cpu, set_op, get_op)
    },
    0xB8..=0xBF => {
      let set_op = operand::Word::reg_index(op0 & 7);
      let get_op = operand::Word::Imm(cpu.memory.next_word());
      set::mov_word(cpu, set_op, get_op)
    },
    0xC2 => {
      let word = cpu.memory.next_word();
      jump::ret(cpu, Some(word))
    },
    0xC3 => jump::ret(cpu, None),
    0xC4 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Word::general(op1);
      let get_op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      set::les_word(cpu, set_op, get_op)
    },
    0xC5 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Word::general(op1);
      let get_op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      set::lds_word(cpu, set_op, get_op)
    },
    0xC6 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Byte::extended(&mut cpu.memory, &cpu.regs, op1);
      let get_op = operand::Byte::Imm(cpu.memory.next_byte());
      set::mov_byte(cpu, set_op, get_op)
    },
    0xC7 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      let get_op = operand::Word::Imm(cpu.memory.next_word());
      set::mov_word(cpu, set_op, get_op)
    },
    0xCA => {
      let word = cpu.memory.next_word();
      jump::retf(cpu, Some(word))
    },
    0xCB => jump::retf(cpu, None),
    0xCC => jump::int(cpu, 3),
    0xCD => {
      let index = cpu.memory.next_byte();
      jump::int(cpu, index)
    },
    0xCE => jump::into(cpu),
    0xCF => jump::iret(cpu),
    0xD0 | 0xD2 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Byte::extended(&mut cpu.memory, &cpu.regs, op1);
      let get_op = if op0 == 0xD0 {
        operand::Byte::Imm(1)
      } else {
        operand::Byte::Reg(register::Byte::CL)
      };
      match (op1 & 0b111000) >> 3 {
        0 => shift::rol_byte(cpu, set_op, get_op),
        1 => shift::ror_byte(cpu, set_op, get_op),
        2 => shift::rcl_byte(cpu, set_op, get_op),
        3 => shift::rcr_byte(cpu, set_op, get_op),
        4 => shift::shl_byte(cpu, set_op, get_op),
        5 => shift::shr_byte(cpu, set_op, get_op),
        6 => panic!("{:05X}: Unknown - {:02X} {:02X}", cpu.current_address, op0, op1),
        7 => shift::sar_byte(cpu, set_op, get_op),
        _ => unreachable!(),
      }
    },
    0xD1 | 0xD3 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      let get_op = if op0 == 0xD1 {
        operand::Byte::Imm(1)
      } else {
        operand::Byte::Reg(register::Byte::CL)
      };
      match (op1 & 0b111000) >> 3 {
        0 => shift::rol_word(cpu, set_op, get_op),
        1 => shift::ror_word(cpu, set_op, get_op),
        2 => shift::rcl_word(cpu, set_op, get_op),
        3 => shift::rcr_word(cpu, set_op, get_op),
        4 => shift::shl_word(cpu, set_op, get_op),
        5 => shift::shr_word(cpu, set_op, get_op),
        6 => panic!("{:05X}: Unknown - {:02X} {:02X}", cpu.current_address, op0, op1),
        7 => shift::sar_word(cpu, set_op, get_op),
        _ => unreachable!(),
      }
    },
    0xD4 => {
      cpu.memory.next_byte();  //Bug in 8086.. It takes an extra byte for no good reason. Doesn't use it.
      bcd::aam(cpu)
    },
    0xD5 => {
      cpu.memory.next_byte();  //Bug in 8086.. It takes an extra byte for no good reason. Doesn't use it.
      bcd::aad(cpu)
    },
    0xD7 => set::xlat(cpu),
    0xE0 => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: LOOPNZ +{:X}", cpu.current_address, offset); }
      jump::loop_relative(cpu, offset, !cpu.flags.zero)
    },
    0xE1 => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: LOOPZ +{:X}", cpu.current_address, offset); }
      jump::loop_relative(cpu, offset, cpu.flags.zero)
    },
    0xE2 => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: LOOP +{:X}", cpu.current_address, offset); }
      jump::loop_relative(cpu, offset, true)
    },
    0xE3 => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JCXZ +{:X}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, cpu.regs.cx == 0)
    },
    0xE4 => {
      let op = operand::Byte::Imm(cpu.memory.next_byte());
      set::in_al_byte(cpu, op, cpu_actor)
    },
    0xE5 => {
      let op = operand::Byte::Imm(cpu.memory.next_byte());
      set::in_ax_byte(cpu, op, cpu_actor)
    },
    0xE6 => {
      let op = operand::Byte::Imm(cpu.memory.next_byte());
      set::out_al_byte(cpu, op)
    },
    0xE7 => {
      let op = operand::Byte::Imm(cpu.memory.next_byte());
      set::out_ax_byte(cpu, op)
    },
    0xE8 => {
      let offset = operand::Word::Imm(cpu.memory.next_word());
      jump::call_relative_word(cpu, offset)
    },
    0xE9 => {
      let offset = cpu.memory.next_word();
      jump::jmp_relative_word(cpu, offset as i16)
    },
    0xEA => {
      let offset = operand::Word::Imm(cpu.memory.next_word());
      let segment = operand::Word::Imm(cpu.memory.next_word());
      jump::jmp_addr(cpu, segment, offset)
    },
    0xEB => {
      let offset = cpu.memory.next_byte() as i8;
      if log_enabled!(Trace) { trace!("{:05X}: JMP +{}", cpu.current_address, offset); }
      jump::jmp_relative(cpu, offset, true)
    },
    0xEC => set::in_al_word(cpu, cpu_actor),
    0xED => set::in_ax_word(cpu, cpu_actor),
    0xEE => set::out_al_word(cpu),
    0xEF => set::out_ax_word(cpu),
    0xF0 => {
      //This is used to make the next instruction atomic.
      //No other chip can read the memory during this time.
      //This is not applicable for us.
      if log_enabled!(Trace) { trace!("{:05X}: LOCK", cpu.current_address); }
      2
    },
    0xF2 => jump::rep(cpu, false, cpu_actor),
    0xF3 => jump::rep(cpu, true, cpu_actor),
    0xF4 => {
      trace!("{:05X}: HLT", cpu.current_address);
      panic!("Stop");
    },
    0xF5 => flag::cmc(cpu),
    0xF6 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Byte::extended(&mut cpu.memory, &cpu.regs, op1);
      match (op1 & 0b111000) >> 3 {
        0 => {
          let get_op = operand::Byte::Imm(cpu.memory.next_byte());
          logic::test_byte(cpu, set_op, get_op)
        },
        1 => panic!("{:05X}: Unknown - {:02X} {:02X}", cpu.current_address, op0, op1),
        2 => logic::not_byte(cpu, set_op),
        3 => math::neg_byte(cpu, set_op),
        4 => math::mul_byte(cpu, set_op),
        5 => math::imul_byte(cpu, set_op),
        6 => math::div_byte(cpu, set_op),
        7 => math::idiv_byte(cpu, set_op),
        _ => unreachable!(),
      }
    },
    0xF7 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      match (op1 & 0b111000) >> 3 {
        0 => {
          let get_op = operand::Word::Imm(cpu.memory.next_word());
          logic::test_word(cpu, set_op, get_op)
        },
        1 => panic!("{:05X}: Unknown - {:02X} {:02X}", cpu.current_address, op0, op1),
        2 => logic::not_word(cpu, set_op),
        3 => math::neg_word(cpu, set_op),
        4 => math::mul_word(cpu, set_op),
        5 => math::imul_word(cpu, set_op),
        6 => math::div_word(cpu, set_op),
        7 => math::idiv_word(cpu, set_op),
        _ => unreachable!(),
      }
    },
    0xF8 => flag::clc(cpu),
    0xF9 => flag::stc(cpu),
    0xFA => flag::cli(cpu),
    0xFB => flag::sti(cpu),
    0xFC => flag::cld(cpu),
    0xFD => flag::std(cpu),
    0xFE => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Byte::extended(&mut cpu.memory, &cpu.regs, op1);
      match (op1 & 0b111000) >> 3 {
        0 => math::inc_byte(cpu, set_op),
        1 => math::dec_byte(cpu, set_op),
        2..=7 => panic!("{:05X}: Unknown - {:02X} {:02X}", cpu.current_address, op0, op1),
        _ => unreachable!(),
      }
    },
    0xFF => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      match (op1 & 0b111000) >> 3 {
        0 => math::inc_word(cpu, set_op),
        1 => math::dec_word(cpu, set_op),
        2 => jump::call_word(cpu, set_op),
        3 => jump::call_far(cpu, set_op),
        4 => jump::jmp_word(cpu, set_op),
        5 => jump::jmp_far(cpu, set_op),
        6 => flag::push(cpu, set_op),
        7 => panic!("{:05X}: Unknown - {:02X} {:02X}", cpu.current_address, op0, op1),
        _ => unreachable!(),
      }
    },
    _ => panic!("{:05X}: Unknown - {:02X}", cpu.current_address, op0),

  };
  
  cpu.memory.current_segment = next_segment; //Remember for the next instruction
  
  cycles
}

//Pattern for many operations:
//0: Eb Gb
//1: Ev Gv
//2: Gb Eb
//3: Gb Ev
//4: AL Ib
//5: AX Iv
pub(crate) fn get_standard_ops(cpu: &mut super::super::CPU, index: u8) -> operand::Pair {
  match index {
    0x00 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Byte::extended(&mut cpu.memory, &cpu.regs, op1);
      let get_op = operand::Byte::general(op1);
      operand::Pair::Bytes(set_op, get_op)
    },
    0x01 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      let get_op = operand::Word::general(op1);
      operand::Pair::Words(set_op, get_op)
    },
    0x02 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Byte::general(op1);
      let get_op = operand::Byte::extended(&mut cpu.memory, &cpu.regs, op1);
      operand::Pair::Bytes(set_op, get_op)
    },
    0x03 => {
      let op1 = cpu.memory.next_byte();
      let set_op = operand::Word::general(op1);
      let get_op = operand::Word::extended(&mut cpu.memory, &cpu.regs, op1);
      operand::Pair::Words(set_op, get_op)
    },
    0x04 => {
      let set_op = operand::Byte::Reg(register::Byte::AL);
      let get_op = operand::Byte::Imm(cpu.memory.next_byte());
      operand::Pair::Bytes(set_op, get_op)
    },
    0x05 => {
      let set_op = operand::Word::Reg(register::Word::AX);
      let get_op = operand::Word::Imm(cpu.memory.next_word());
      operand::Pair::Words(set_op, get_op)
    },
    _ => unreachable!(),
  }
}
