use super::memory::Memory;
use super::register::Registers;
use super::memory;
use super::register;

pub(crate) enum Pair {
  Bytes(Byte, Byte),
  Words(Word, Word),
}

pub(crate) enum Byte {
  Mem{addr: u16, label: String, cycles: usize},
  Reg(register::Byte),
  Imm(u8),
}

pub(crate) enum Word {
  Mem{addr: u16, label: String, cycles: usize},
  Reg(register::Word),
  Seg(memory::Segment),
  Imm(u16),
}

impl Byte {
  pub(crate) fn reg_index(nib: u8) -> Byte {
    let reg = match nib & 0b111 { //Doing & 111b is not necessary. I am doing it just in case.
      0 => register::Byte::AL,
      1 => register::Byte::CL,
      2 => register::Byte::DL,
      3 => register::Byte::BL,
      4 => register::Byte::AH,
      5 => register::Byte::CH,
      6 => register::Byte::DH,
      7 => register::Byte::BH,
      _ => unreachable!(),
    };
    Byte::Reg(reg)
  }

  pub(crate) fn label(&self) -> String {
    match self {
      Byte::Mem{label, ..} => format!("[{}]", label),
      Byte::Reg(reg) => match reg {
        register::Byte::AL => "AL".to_string(), register::Byte::CL => "CL".to_string(), register::Byte::DL => "DL".to_string(), register::Byte::BL => "BL".to_string(),
        register::Byte::AH => "AH".to_string(), register::Byte::CH => "CH".to_string(), register::Byte::DH => "DH".to_string(), register::Byte::BH => "BH".to_string(),
      },
      Byte::Imm(num) => format!("{:X}", num),
    }
  }

  //Gb
  pub(crate) fn general(op1: u8) -> Byte {
    let index = (op1 & 0b111000) >> 3;
    Byte::reg_index(index)
  }
  
  //Ob
  pub(crate) fn address(memory: &mut Memory) -> Byte {
    let addr = memory.next_word();
    let label = format!("{:X}", addr);
    Byte::Mem{addr, label, cycles: 5}
  }
  
  //Eb
  pub(crate) fn extended(memory: &mut Memory, regs: &Registers, op1: u8) -> Byte {
    let ind1 = op1 & 7;
    match op1 {
      0x00..=0xBF => {
        let (mut addr, mut label, mut cycles) = match ind1 {
          0 => (regs.bx + regs.si, "BX+SI".to_string(), 7),
          1 => (regs.bx + regs.di, "BX+DI".to_string(), 8),
          2 => (regs.bp + regs.si, "BP+SI".to_string(), 8),
          3 => (regs.bp + regs.di, "BP+DI".to_string(), 7),
          4 => (regs.si, "SI".to_string(), 5),
          5 => (regs.di, "DI".to_string(), 5),
          6 => (regs.bp, "BP".to_string(), 5),
          7 => (regs.bx, "BX".to_string(), 5),
          _ => unreachable!(),
        };
        match op1 {
          0x00..=0x3F => {
            if ind1 == 0x6 { //Special case replacing bp
              addr = memory.next_word();
              label = format!("{:X}", addr);
            }
          },
          0x40..=0x7F => {
            let offset = memory.next_byte() as u16;
            addr += offset;
            label = format!("{}+{:X}", label, offset);
            cycles += 6;
          },
          0x80..=0xBF => {
            let offset = memory.next_word();
            addr += offset;
            label = format!("{}+{:X}", label, offset);
            cycles += 6;
          },
          _ => unreachable!(),
        };
        Byte::Mem{addr, label, cycles}
      },
      0xC0..=0xFF => {
        Byte::reg_index(ind1)
      }
    }
  }

  pub(crate) fn get_cycles(&self) -> usize {
    match self {
      Byte::Reg(_) => 2,
      Byte::Mem{cycles, ..} => 23+cycles,
      Byte::Imm(_) => unreachable!("This should be impossible. Cycles for an immediate..")
    }
  }

  pub(crate) fn get_rotate_cycles(&self, get_op: &Byte, get_val: u8) -> usize {
    let mut cycles = self.get_cycles();
    if let Byte::Reg(_) = get_op {
      cycles += 5 + 4 * get_val as usize;
    }
    cycles  
  }

  pub(crate) fn get_cycles_fast(&self, get_op: &Byte) -> usize {
    let set_op = self;
    match (set_op, get_op) {
      (Byte::Reg(_), Byte::Reg(_)) => 2,
      (Byte::Reg(_), Byte::Imm(_)) => 4,
      (Byte::Mem{cycles, ..}, Byte::Reg(_)) => 13 + cycles,
      (Byte::Mem{cycles, ..}, Byte::Imm(_)) => 14 + cycles,
      (Byte::Reg(_), Byte::Mem{cycles, ..}) => 12 + cycles,
      (Byte::Imm(_), _) => unreachable!("This should be impossible. SetOp is an immediate"),
      (Byte::Mem{..}, Byte::Mem{..}) => unreachable!("This should be impossible. Set mem to mem."),
    }
  }
  
  pub(crate) fn get_cycles_slow(&self, get_op: &Byte) -> usize {
    let set_op = self;
    match (set_op, get_op) {
      (Byte::Reg(_), Byte::Reg(_)) => 3,
      (Byte::Reg(_), Byte::Imm(_)) => 4,
      (Byte::Mem{cycles, ..}, Byte::Reg(_)) => 24 + cycles,
      (Byte::Mem{cycles, ..}, Byte::Imm(_)) => 23 + cycles,
      (Byte::Reg(_), Byte::Mem{cycles, ..}) => 13 + cycles,
      (Byte::Imm(_), _) => unreachable!("This should be impossible. SetOp is an immediate"),
      (Byte::Mem{..}, Byte::Mem{..}) => unreachable!("This should be impossible. Set mem to mem."),
    }
  }
}

impl Word {
  pub(crate) fn reg_index(index: u8) -> Word {
    let reg = match index & 7 { //Doing & 7 is not necessary. I am doing it just in case.
      0 => register::Word::AX,
      1 => register::Word::CX,
      2 => register::Word::DX,
      3 => register::Word::BX,
      4 => register::Word::SP,
      5 => register::Word::BP,
      6 => register::Word::SI,
      7 => register::Word::DI,
      _ => unreachable!(),
    };
    Word::Reg(reg)
  }

  pub(crate) fn label(&self) -> String {
    match self {
      Word::Mem{label, ..} => format!("[{}]", label),
      Word::Reg(reg) => match reg {
        register::Word::AX => "AX".to_string(), register::Word::CX => "CX".to_string(), register::Word::DX => "DX".to_string(), register::Word::BX => "BX".to_string(),
        register::Word::SP => "SP".to_string(), register::Word::BP => "BP".to_string(), register::Word::SI => "SI".to_string(), register::Word::DI => "DI".to_string(),
      },
      Word::Seg(seg) => match seg {
        memory::Segment::ES => "ES".to_string(), memory::Segment::CS => "CS".to_string(), memory::Segment::SS => "SS".to_string(), memory::Segment::DS => "DS".to_string(),
      },
      Word::Imm(num) => format!("{:X}", num),
    }
  }

  
  //Gw or Gv
  pub(crate) fn general(op1: u8) -> Word {
    let index = (op1 & 0b111000) >> 3;
    Word::reg_index(index)
  }
  
  //Ov
  pub(crate) fn address(memory: &mut Memory) -> Word {
    let addr = memory.next_word();
    let label = format!("{:X}", addr);
    Word::Mem{addr, label, cycles: 5}
  }
  
  //Ew or Ev
  pub(crate) fn extended(memory: &mut Memory, regs: &Registers, op1: u8) -> Word {
    let ind1 = op1 & 7;
    match op1 {
      0x00..=0xBF => {
        let (mut addr, mut label, mut cycles) = match ind1 {
          0 => (regs.bx + regs.si, "BX+SI".to_string(), 7),
          1 => (regs.bx + regs.di, "BX+DI".to_string(), 8),
          2 => (regs.bp + regs.si, "BP+SI".to_string(), 8),
          3 => (regs.bp + regs.di, "BP+DI".to_string(), 7),
          4 => (regs.si, "SI".to_string(), 5),
          5 => (regs.di, "DI".to_string(), 5),
          6 => (regs.bp, "BP".to_string(), 5),
          7 => (regs.bx, "BX".to_string(), 5),
          _ => unreachable!(),
        };
        match op1 {
          0x00..=0x3F => {
            if ind1 == 0x6 { //Special case replacing bp
              addr = memory.next_word();
              label = format!("{:X}", addr);
            }
          },
          0x40..=0x7F => {
            let offset = memory.next_byte() as u16;
            addr += offset;
            label = format!("{}+{:X}", label, offset);
            cycles += 6;
          },
          0x80..=0xBF => {
            let offset = memory.next_word();
            addr += offset;
            label = format!("{}+{:X}", label, offset);
            cycles += 6;
          },
          _ => unreachable!(),
        };
        Word::Mem{addr, label, cycles}
      },
      0xC0..=0xFF => {
        Word::reg_index(ind1)
      }
    }
  }

  pub(crate) fn seg_index(index: u8) -> Word {
    let seg = match index & 0b11 { //Doing & 0b11 is not necessary. I am doing it just in case.
      0 => memory::Segment::ES,
      1 => memory::Segment::CS,
      2 => memory::Segment::SS,
      3 => memory::Segment::DS,
      _ => unreachable!(),
    };
    Word::Seg(seg)
  }

  //Sw
  pub(crate) fn segment(op1: u8) -> Word {
    let index = (op1 & 0b11000) >> 3;
    Word::seg_index(index)
  }

  pub(crate) fn get_cycles(&self) -> usize {
    match self {
      Word::Reg(_) | Word::Seg(_) => 3,
      Word::Mem{cycles, ..} => 24 + cycles,
      Word::Imm(_) => unreachable!("This should be impossible. Cycles for an immediate..")
    }
  }

  pub(crate) fn get_rotate_cycles(&self, get_op: &Byte, get_val: u8) -> usize {
    let mut cycles = self.get_cycles();
    if let Byte::Reg(_) = get_op {
      cycles += 5 + 4 * get_val as usize;
    }
    cycles  
  }

  pub(crate) fn get_cycles_fast(&self, get_op: &Word) -> usize {
    let set_op = self;
    match (set_op, get_op) {
      (Word::Seg(_), Word::Reg(_)) | (Word::Reg(_), Word::Seg(_)) | (Word::Reg(_), Word::Reg(_)) => 2,
      (Word::Seg(_), Word::Mem{cycles, ..}) => 12 + cycles,
      (Word::Mem{cycles, ..}, Word::Seg(_)) => 11 + cycles,
      (Word::Seg(_), _) | (_, Word::Seg(_)) => unreachable!("This should be impossible. Segment register with memory or immediate."),
      (Word::Reg(_), Word::Imm(_)) => 4,
      (Word::Mem{cycles, ..}, Word::Reg(_)) => 13 + cycles,
      (Word::Mem{cycles, ..}, Word::Imm(_)) => 14 + cycles,
      (Word::Reg(_), Word::Mem{cycles, ..}) => 12 + cycles,
      (Word::Imm(_), _) => unreachable!("This should be impossible. SetOp is an immediate"),
      (Word::Mem{..}, Word::Mem{..}) => unreachable!("This should be impossible. Set mem to mem."),
    }
  }
  
  pub(crate) fn get_cycles_slow(&self, get_op: &Word) -> usize {
    let set_op = self;
    match (set_op, get_op) {
      (Word::Reg(_), Word::Reg(_)) => 3,
      (Word::Reg(_), Word::Imm(_)) => 4,
      (Word::Mem{cycles, ..}, Word::Reg(_)) => 24 + cycles,
      (Word::Mem{cycles, ..}, Word::Imm(_)) => 23 + cycles,
      (Word::Reg(_), Word::Mem{cycles, ..}) => 13 + cycles,
      (Word::Imm(_), _) => unreachable!("This should be impossible. SetOp is an immediate"),
      (Word::Mem{..}, Word::Mem{..}) => unreachable!("This should be impossible. Set mem to mem."),
      (Word::Seg(_), _) | (_, Word::Seg(_)) => unreachable!("This should be impossible. Any slow Segment instructions."),
    }
  }
}
