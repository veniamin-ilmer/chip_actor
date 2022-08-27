pub(crate) struct Memory {
  pub(crate) es: u16,  //Extra
  pub(crate) cs: u16,  //Code
  pub(crate) ss: u16,  //Stack
  pub(crate) ds: u16,  //Data
  pub(crate) ip: u16,  //Instruction
  
  pub(crate) current_segment: Segment,
  pub(crate) current_instruction: u64,
  
  pub(crate) ram: Vec<u8>,
}

pub(crate) enum Segment {
  ES, CS, SS, DS,
}

pub(crate) fn calculate_addr(segment: u16, offset: u16) -> usize {
  let segment = (segment as usize) << 4;
  segment + offset as usize
}

impl Memory {

  pub(crate) fn next_byte(&mut self) -> u8 {
    (*self).ip += 1;
    let byte = self.current_instruction & 0xFF;
    self.current_instruction >>= 8;
    byte as u8
  }

  pub(crate) fn next_word(&mut self) -> u16 {
    (*self).ip += 2;
    let word = self.current_instruction & 0xFFFF;
    self.current_instruction >>= 2*8;
    word as u16
  }

  pub(crate) fn set_byte(&mut self, offset: u16, byte: u8) {
    let addr = calculate_addr(self.get_seg(&self.current_segment), offset);
    self.ram[addr] = byte;
  }
  
  pub(crate) fn get_byte(&self, offset: u16) -> u8 {
    let addr = calculate_addr(self.get_seg(&self.current_segment), offset);
    self.ram[addr]
  }

  pub(crate) fn set_word(&mut self, offset: u16, word: u16) {
    let addr = calculate_addr(self.get_seg(&self.current_segment), offset);
    let bytes = word.to_le_bytes();
    let (_left, right) = self.ram.split_at_mut(addr);
    let (middle, _) = right.split_at_mut(2);
    middle.copy_from_slice(&bytes[..]);
  }

  pub(crate) fn get_word(&self, offset: u16) -> u16 {
    let addr = calculate_addr(self.get_seg(&self.current_segment), offset);
    self.get_word_at_addr(addr)
  }
  pub(crate) fn get_word_at_addr(&self, addr: usize) -> u16 {
    let slice = &self.ram[addr..addr+2];
    u16::from_le_bytes(slice.try_into().unwrap())
  }

  pub(crate) fn prepare_next_instruction(&mut self) {
    let addr = calculate_addr(self.cs, self.ip);
    let slice = &self.ram[addr..addr+8];
    self.current_instruction = u64::from_le_bytes(slice.try_into().unwrap());
  }
  
  pub(crate) fn get_current_address(&self) -> usize {
    calculate_addr(self.cs, self.ip)
  }
  
  pub(crate) fn set_seg(&mut self, seg: &Segment, value: u16) {
    match seg {
      Segment::ES => self.es = value, Segment::CS => self.cs = value, Segment::SS => self.ss = value, Segment::DS => self.ds = value,
    }
  }
  pub(crate) fn get_seg(&self, seg: &Segment) -> u16 {
    match seg {
      Segment::ES => self.es, Segment::CS => self.cs, Segment::SS => self.ss, Segment::DS => self.ds,
    }
  }
}