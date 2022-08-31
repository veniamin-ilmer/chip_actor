#[derive(Default)]
pub(crate) struct Flags {
  pub(crate) carry: bool,
  pub(crate) parity: bool,
  pub(crate) adjust: bool,
  pub(crate) zero: bool,
  pub(crate) sign: bool,
  pub(crate) trap: bool,
  pub(crate) interrupt: bool,
  pub(crate) direction: bool,
  pub(crate) overflow: bool,
}

impl Flags {
  
  pub(crate) fn test_and_or_xor_byte(&mut self, result: u8) {
    self.carry = false;
    self.parity_zero_sign_byte(result);
    //self.adjust undefined - TODO - what does this mean?
    self.overflow = false;
  }
  pub(crate) fn test_and_or_xor_word(&mut self, result: u16) {
    self.carry = false;
    self.parity_zero_sign_word(result);
    //self.adjust undefined - TODO - what does this mean?
    self.overflow = false;
  }

  pub(crate) fn inc_byte(&mut self, value: u8) -> u8 {
    let result = value.wrapping_add(1);
    self.parity_zero_sign_byte(result);
    self.adjust = value & 0xF == 0xF; //This only works for increment
    self.overflow = result == 0x80; //This only works for increment
    result
  }
  pub(crate) fn inc_word(&mut self, value: u16) -> u16 {
    let result = value.wrapping_add(1);
    self.parity_zero_sign_word(result);
    self.adjust = value & 0xF == 0xF; //This only works for increment
    self.overflow = result == 0x8000; //This only works for increment
    result
  }

  pub(crate) fn dec_byte(&mut self, value: u8) -> u8 {
    let result = value.wrapping_sub(1);
    self.parity_zero_sign_byte(result);
    self.adjust = result & 0xF == 0xF; //This only works for decrement
    self.overflow = value == 0x80; //This only works for decrement
    result
  }
  pub(crate) fn dec_word(&mut self, value: u16) -> u16 {
    let result = value.wrapping_sub(1);
    self.parity_zero_sign_word(result);
    self.adjust = result & 0xF == 0xF; //This only works for decrement
    self.overflow = value == 0x8000; //This only works for decrement
    result
  }

  pub(crate) fn shr_byte(&mut self, set_val: u8, get_val: u8) -> u8 {
    let result = self.ror_rcr_byte(set_val, get_val);
    self.parity_zero_sign_byte(result);
    result
  }
  pub(crate) fn shr_word(&mut self, set_val: u16, get_val: u8) -> u16 {
    let result = self.ror_rcr_word(set_val, get_val);
    self.parity_zero_sign_word(result);
    result
  }
  pub(crate) fn ror_rcr_byte(&mut self, set_val: u8, get_val: u8) -> u8 {
    if get_val == 0 {
      return set_val;
    }

    let prev_sign = set_val & 0b1000_0000;
    
    let mut result = set_val >> (get_val - 1);
    self.carry = result & 1 != 0;
    result >>= 1;
    
    let new_sign = result & 0b1000_0000;
    self.overflow = prev_sign != new_sign;
    result
  }
  pub(crate) fn ror_rcr_word(&mut self, set_val: u16, get_val: u8) -> u16 {
    if get_val == 0 {
      return set_val;
    }

    let prev_sign = set_val & 0b1000_0000_0000_0000;
    
    let mut result = set_val >> (get_val - 1);
    self.carry = result & 1 != 0;
    result >>= 1;
    
    let new_sign = result & 0b1000_0000_0000_0000;
    self.overflow = prev_sign != new_sign;
    result
  }
  
  pub(crate) fn shl_byte(&mut self, set_val: u8, get_val: u8) -> u8 {
    let result = self.rol_rcl_byte(set_val, get_val);
    self.parity_zero_sign_byte(result);
    result
  }
  pub(crate) fn shl_word(&mut self, set_val: u16, get_val: u8) -> u16 {
    let result = self.rol_rcl_word(set_val, get_val);
    self.parity_zero_sign_word(result);
    result
  }

  pub(crate) fn rol_rcl_byte(&mut self, set_val: u8, get_val: u8) -> u8 {
    if get_val == 0 {
      return set_val;
    }

    let prev_sign = set_val & 0b1000_0000;
    
    let mut result = set_val << (get_val - 1);
    self.carry = result & 0b1000_0000 != 0;
    result <<= 1;
    
    let new_sign = result & 0b1000_0000;
    self.overflow = prev_sign != new_sign;
    result
  }
  pub(crate) fn rol_rcl_word(&mut self, set_val: u16, get_val: u8) -> u16 {
    if get_val == 0 {
      return set_val;
    }
    
    let prev_sign = set_val & 0b1000_0000_0000_0000;
    
    let mut result = set_val << (get_val - 1);
    self.carry = result & 0b1000_0000_0000_0000 != 0;
    result <<= 1;
    
    let new_sign = result & 0b1000_0000_0000_0000;
    self.overflow = prev_sign != new_sign;
    result
  }

  pub(crate) fn add_byte(&mut self, set_val: u8, get_val: u8) -> u8 {
    let (result, carry) = set_val.overflowing_add(get_val);
    self.carry = carry;
    self.parity_zero_sign_byte(result);
    self.adjust = (set_val & 0xF) > (result & 0xF); //TODO - Confirm this, especially with negative / wrapping
    let (_, overflow) = (set_val as i8).overflowing_add(get_val as i8);
    self.overflow = overflow;
    result
  }
  pub(crate) fn add_word(&mut self, set_val: u16, get_val: u16) -> u16 {
    let (result, carry) = set_val.overflowing_add(get_val);
    self.carry = carry;
    self.parity_zero_sign_word(result);
    self.adjust = (set_val & 0xF) > (result & 0xF); //TODO - Confirm this, especially with negative / wrapping
    let (_, overflow) = (set_val as i16).overflowing_add(get_val as i16);
    self.overflow = overflow;
    result
  }

  pub(crate) fn adc_byte(&mut self, set_val: u8, get_val: u8) -> u8 {
    let (r, overflow1) = (set_val as i8).overflowing_add(get_val as i8);
    if self.carry {
      let (_, overflow2) = (r as i8).overflowing_add(1);
      self.overflow = overflow1 | overflow2;
    } else {
      self.overflow = overflow1;
    }
    let (mut result, carry1) = set_val.overflowing_add(get_val);
    if self.carry {
      let (r, carry2) = result.overflowing_add(1);
      self.carry = carry1 | carry2;
      result = r;
    } else {
      self.carry = carry1;
    }
    self.parity_zero_sign_byte(result);
    self.adjust = (set_val & 0xF) > (result & 0xF); //TODO - Confirm this, especially with negative / wrapping
    result
  }
  pub(crate) fn adc_word(&mut self, set_val: u16, get_val: u16) -> u16 {
    let (r, overflow1) = (set_val as i16).overflowing_add(get_val as i16);
    if self.carry {
      let (_, overflow2) = (r as i16).overflowing_add(1);
      self.overflow = overflow1 | overflow2;
    } else {
      self.overflow = overflow1;
    }
    let (mut result, carry1) = set_val.overflowing_add(get_val);
    if self.carry {
      let (r, carry2) = result.overflowing_add(1);
      self.carry = carry1 | carry2;
      result = r;
    } else {
      self.carry = carry1;
    }
    self.parity_zero_sign_word(result);
    self.adjust = (set_val & 0xF) > (result & 0xF); //TODO - Confirm this, especially with negative / wrapping
    result
  }
  
  pub(crate) fn cmp_sub_byte(&mut self, set_val: u8, get_val: u8) -> u8 {
    let (result, carry) = set_val.overflowing_sub(get_val);
    self.carry = carry;
    self.parity_zero_sign_byte(result);
    self.adjust = (set_val & 0xF) < (result & 0xF); //TODO - Confirm this, especially with negative / wrapping
    let (_, overflow) = (set_val as i8).overflowing_sub(get_val as i8);
    self.overflow = overflow;
    result
  }
  pub(crate) fn cmp_sub_word(&mut self, set_val: u16, get_val: u16) -> u16 {
    let (result, carry) = set_val.overflowing_sub(get_val);
    self.carry = carry;
    self.parity_zero_sign_word(result);
    self.adjust = (set_val & 0xF) < (result & 0xF); //TODO - Confirm this, especially with negative / wrapping
    let (_, overflow) = (set_val as i16).overflowing_sub(get_val as i16);
    self.overflow = overflow;
    result
  }
  
  pub(crate) fn sbb_byte(&mut self, set_val: u8, get_val: u8) -> u8 {
    let (r, overflow1) = (set_val as i8).overflowing_sub(get_val as i8);
    if self.carry {
      let (_, overflow2) = (r as i8).overflowing_sub(1);
      self.overflow = overflow1 | overflow2;
    } else {
      self.overflow = overflow1;
    }
    let (mut result, carry1) = set_val.overflowing_sub(get_val);
    if self.carry {
      let (r, carry2) = result.overflowing_sub(1);
      self.carry = carry1 | carry2;
      result = r;
    } else {
      self.carry = carry1;
    }
    self.parity_zero_sign_byte(result);
    self.adjust = (set_val & 0xF) < (result & 0xF); //TODO - Confirm this, especially with negative / wrapping
    result
  }
  pub(crate) fn sbb_word(&mut self, set_val: u16, get_val: u16) -> u16 {
    let (r, overflow1) = (set_val as i16).overflowing_sub(get_val as i16);
    if self.carry {
      let (_, overflow2) = (r as i16).overflowing_sub(1);
      self.overflow = overflow1 | overflow2;
    } else {
      self.overflow = overflow1;
    }
    let (mut result, carry1) = set_val.overflowing_sub(get_val);
    if self.carry {
      let (r, carry2) = result.overflowing_sub(1);
      self.carry = carry1 | carry2;
      result = r;
    } else {
      self.carry = carry1;
    }
    self.parity_zero_sign_word(result);
    self.adjust = (set_val & 0xF) < (result & 0xF); //TODO - Confirm this, especially with negative / wrapping
    result
  }
  
  pub(crate) fn get_bits_byte(&self) -> u8 {
    let mut value = 0b0000_0010;  //All 1s here are always set on a 8086 and 186.
    if self.carry { value |= 0b1; }
    if self.parity { value |= 0b100; }
    if self.adjust { value |= 0b1_0000; }
    if self.zero { value |= 0b100_0000; }
    if self.sign { value |= 0b1000_0000; }
    value
  }
  
  pub(crate) fn get_bits_word(&self) -> u16 {
    let mut value = self.get_bits_byte() as u16;
    value |= 0b1111_0000_0000_0010;  //All 1s here are always set on a 8086 and 186.
    if self.trap { value |= 0b1_0000_0000; }
    if self.interrupt { value |= 0b10_0000_0000; }
    if self.direction { value |= 0b100_0000_0000; }
    if self.overflow { value |= 0b1000_0000_0000; }
    value
  }

  pub(crate) fn set_bits_byte(&mut self, value: u8) {
    self.carry = value & 0b1 != 0;
    self.parity = value & 0b100 != 0;
    self.adjust = value & 0b1_0000 != 0;
    self.zero = value & 0b100_0000 != 0;
    self.sign = value & 0b1000_0000 != 0;
  }
  
  pub(crate) fn set_bits_word(&mut self, value: u16) {
    self.set_bits_byte(value as u8);
    self.trap = value & 0b1_0000_0000 != 0;
    self.interrupt = value & 0b10_0000_0000 != 0;
    self.direction = value & 0b100_0000_0000 != 0;
    self.overflow = value & 0b1000_0000_0000 != 0;
  }
  
  pub(crate) fn parity_zero_sign_byte(&mut self, result: u8) {
    self.parity_byte(result);
    self.zero_byte(result);
    self.sign_byte(result);
  }
  pub(crate) fn parity_zero_sign_word(&mut self, result: u16) {
    self.parity_word(result);
    self.zero_word(result);
    self.sign_word(result);
  }
  
  fn parity_byte(&mut self, result: u8) {
    let mut ret = result;
    ret ^= ret >> 4;
    ret ^= ret >> 2;
    ret ^= ret >> 1;
    self.parity = (ret & 1) == 0;
  }
  fn parity_word(&mut self, result: u16) { //8086 just looking at first byte. Ignoring remainder.
    let mut ret = result;
    ret ^= ret >> 4;
    ret ^= ret >> 2;
    ret ^= ret >> 1;
    self.parity = (ret & 1) == 0;
  }

  fn zero_byte(&mut self, result: u8) {
    self.zero = result == 0;
  }
  fn zero_word(&mut self, result: u16) {
    self.zero = result == 0;
  }

  fn sign_byte(&mut self, result: u8) {
    let signed = result as i8;
    self.sign = signed < 0;
  }
  fn sign_word(&mut self, result: u16) {
    let signed = result as i16;
    self.sign = signed < 0;
  }
}
