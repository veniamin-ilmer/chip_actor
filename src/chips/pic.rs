use stakker::*;
use crate::Board;

//Intel 8259 - Programmable Interrupt Controller (PIC)
//https://www.stanislavs.org/helppc/8259.html
//https://wiki.osdev.org/PIC

use log::debug;

#[derive(Debug, Default)]
enum VectorType {
  #[default]
  Bytes4,
  Bytes8,
}

#[derive(Debug, Default)]
enum Trigger {
  #[default]
  Level,
  Edge,
}

#[derive(Debug, Default)]
pub(crate) struct IRQ {
  master: bool, //true=master, false=slave
  enabled: bool,
  interrupt_requested: bool,
  interrupted_cpu: bool,
}

#[derive(Debug, Default)]
enum RegisterType {
  #[default]
  IRR,  //Interrupt Request Register - Interrupt Requested but not interrupted cpu.
  ISR,  //In-Service Register - Interrupted CPU.
}

pub(crate) struct PIC {
  icw4_needed: bool,  //Gives additional information about the environment
  single: bool,  //true = single chip. false = cascade.
  vector_type: VectorType,
  trigger: Trigger,
  next_set_index: u8,
  next_get: RegisterType,
  vector_offset: u8,
  irq0: IRQ,
  irq1: IRQ,
  irq2: IRQ,
  irq3: IRQ,
  irq4: IRQ,
  irq5: IRQ,
  irq6: IRQ,
  irq7: IRQ,
  board: Actor<Board>,
}

impl PIC {
  pub(crate) fn init(_: CX![], board: Actor<Board>) -> Option<Self> {
    Some(Self {
      next_set_index: 5,  //By default, we are just setting enabled/disabled.
      vector_offset: 0b1000,  //By default, the PIC uses interrupts 0x08 to 0x0F.
      trigger: Default::default(),
      single: false,
      icw4_needed: false,
      next_get: Default::default(),
      vector_type: Default::default(),
      irq0: Default::default(),
      irq1: Default::default(),
      irq2: Default::default(),
      irq3: Default::default(),
      irq4: Default::default(),
      irq5: Default::default(),
      irq6: Default::default(),
      irq7: Default::default(),
      board,
    })
  }
  
  pub(crate) fn in_port_1(&mut self, _: CX![], ret: Ret<u8>) {
    ret!([ret], match self.next_get {
      RegisterType::IRR => self.get_interrupt_requested(),
      RegisterType::ISR => self.get_interrupted_cpu(),
    })
  }

  pub(crate) fn out_port_1(&mut self, _: CX![], register: u8) {
    match ((register & 0b1_0000) >> 4, (register & 0b1000) >> 3) {
      (0, 0) => self.operation_control_2(register),
      (0, 1) => self.operation_control_3(register),
      _ => self.initialization_1(register),
    }
  }
  
  pub(crate) fn out_port_2(&mut self, _: CX![], register: u8) {
    match self.next_set_index {
      2 => self.initialization_2(register),
      3 => self.initialization_3(register),
      4 => self.initialization_4(register),
      _ => self.operation_control_1(register),
    }
  }

  fn initialization_1(&mut self, register: u8) {
    self.icw4_needed = matches!(register & 0b1, 0b1);
    self.single = matches!(register & 0b10, 0b10);
    self.vector_type = if matches!(register & 0b100, 0b100) { VectorType::Bytes4 } else { VectorType::Bytes8 };
    self.trigger = if matches!(register & 0b1000, 0b1000) { Trigger::Level } else { Trigger::Edge };
    self.next_set_index = 2;
    debug!("PIC Init1");
  }
  
  fn initialization_2(&mut self, register: u8) {
    self.vector_offset = register;
    debug!("PIC Init2 offset: {:?}", self.vector_offset);
    self.next_set_index = 3;
  }

  fn initialization_3(&mut self, register: u8) {
    self.irq0.master = matches!(register & 0b1, 0);
    self.irq1.master = matches!(register & 0b10, 0);
    self.irq2.master = matches!(register & 0b100, 0);
    self.irq3.master = matches!(register & 0b1000, 0);
    self.irq4.master = matches!(register & 0b1_0000, 0);
    self.irq5.master = matches!(register & 0b10_0000, 0);
    self.irq6.master = matches!(register & 0b100_0000, 0);
    self.irq7.master = matches!(register & 0b1000_0000, 0);
    debug!("PIC Init3");
    if self.icw4_needed {
      self.next_set_index = 4;
    } else {  //Skip over it
      self.next_set_index = 5;
    }  
  }

  fn initialization_4(&mut self, _register: u8) {
    /*
    I don't really care about this for now..
    #define ICW4_8086	0x01		/* 8086/88 (MCS-80/85) mode */
    #define ICW4_AUTO	0x02		/* Auto (normal) EOI */
    #define ICW4_BUF_SLAVE	0x08		/* Buffered mode/slave */
    #define ICW4_BUF_MASTER	0x0C		/* Buffered mode/master */
    #define ICW4_SFNM	0x10		/* Special fully nested (not) */
    */
    debug!("PIC Init4");
    self.next_set_index = 5;
  }
  
  //Set IRQs Enabled
  fn operation_control_1(&mut self, register: u8) {
    self.irq0.enabled = matches!(register & 0b1, 0);
    self.irq1.enabled = matches!(register & 0b10, 0);
    self.irq2.enabled = matches!(register & 0b100, 0);
    self.irq3.enabled = matches!(register & 0b1000, 0);
    self.irq4.enabled = matches!(register & 0b1_0000, 0);
    self.irq5.enabled = matches!(register & 0b10_0000, 0);
    self.irq6.enabled = matches!(register & 0b100_0000, 0);
    self.irq7.enabled = matches!(register & 0b1000_0000, 0);
    debug!("IRQ Enabled 0: {}, 1: {}, 2: {}, 3: {}, 4: {}, 5: {}, 6: {}, 7: {}",
          self.irq0.enabled, self.irq1.enabled, self.irq2.enabled, self.irq3.enabled, self.irq4.enabled, self.irq5.enabled, self.irq6.enabled, self.irq7.enabled);
    //Check if there are any newly enabled IRQs which were waiting to interrupt, and call them.
    //TODO - Fill in the rest of the interrupts here.
  }
  
  pub(crate) fn get_irqs_enabled(&self, _: CX![], ret: Ret<u8>) {
    let mut result = 0;
    if !self.irq0.enabled { result |= 0b1; }
    if !self.irq1.enabled { result |= 0b10; }
    if !self.irq2.enabled { result |= 0b100; }
    if !self.irq3.enabled { result |= 0b1000; }
    if !self.irq4.enabled { result |= 0b1_0000; }
    if !self.irq5.enabled { result |= 0b10_0000; }
    if !self.irq6.enabled { result |= 0b100_0000; }
    if !self.irq7.enabled { result |= 0b1000_0000; }
    debug!("IRQ Enabled 0: {}, 1: {}, 2: {}, 3: {}, 4: {}, 5: {}, 6: {}, 7: {}",
          self.irq0.enabled, self.irq1.enabled, self.irq2.enabled, self.irq3.enabled, self.irq4.enabled, self.irq5.enabled, self.irq6.enabled, self.irq7.enabled);
    ret!([ret], result)
  }
  
  fn get_interrupt_requested(&self) -> u8 {
    let mut result = 0;
    if self.irq0.interrupt_requested { result |= 0b1; }
    if self.irq1.interrupt_requested { result |= 0b10; }
    if self.irq2.interrupt_requested { result |= 0b100; }
    if self.irq3.interrupt_requested { result |= 0b1000; }
    if self.irq4.interrupt_requested { result |= 0b1_0000; }
    if self.irq5.interrupt_requested { result |= 0b10_0000; }
    if self.irq6.interrupt_requested { result |= 0b100_0000; }
    if self.irq7.interrupt_requested { result |= 0b1000_0000; }
    debug!("IRR 0: {}, 1: {}, 2: {}, 3: {}, 4: {}, 5: {}, 6: {}, 7: {}",
          self.irq0.interrupt_requested, self.irq1.interrupt_requested, self.irq2.interrupt_requested, self.irq3.interrupt_requested,
          self.irq4.interrupt_requested, self.irq5.interrupt_requested, self.irq6.interrupt_requested, self.irq7.interrupt_requested);
    result
  }

  fn get_interrupted_cpu(&self) -> u8 {
    let mut result = 0;
    if self.irq0.interrupted_cpu { result |= 0b1; }
    if self.irq1.interrupted_cpu { result |= 0b10; }
    if self.irq2.interrupted_cpu { result |= 0b100; }
    if self.irq3.interrupted_cpu { result |= 0b1000; }
    if self.irq4.interrupted_cpu { result |= 0b1_0000; }
    if self.irq5.interrupted_cpu { result |= 0b10_0000; }
    if self.irq6.interrupted_cpu { result |= 0b100_0000; }
    if self.irq7.interrupted_cpu { result |= 0b1000_0000; }
    debug!("ISR 0: {}, 1: {}, 2: {}, 3: {}, 4: {}, 5: {}, 6: {}, 7: {}",
          self.irq0.interrupted_cpu, self.irq1.interrupted_cpu, self.irq2.interrupted_cpu, self.irq3.interrupted_cpu,
          self.irq4.interrupted_cpu, self.irq5.interrupted_cpu, self.irq6.interrupted_cpu, self.irq7.interrupted_cpu);
    result
  }
  
  //End Of Interrupt
  fn operation_control_2(&mut self, register: u8) {
    debug!("Got an End Of Interrupt command: {:X}", register);
    self.irq0.interrupted_cpu = false;
    self.irq1.interrupted_cpu = false;
    self.irq2.interrupted_cpu = false;
    self.irq3.interrupted_cpu = false;
    self.irq4.interrupted_cpu = false;
    self.irq5.interrupted_cpu = false;
    self.irq6.interrupted_cpu = false;
    self.irq7.interrupted_cpu = false;
    self.irq0.interrupt_requested = false;
    self.irq1.interrupt_requested = false;
    self.irq2.interrupt_requested = false;
    self.irq3.interrupt_requested = false;
    self.irq4.interrupt_requested = false;
    self.irq5.interrupt_requested = false;
    self.irq6.interrupt_requested = false;
    self.irq7.interrupt_requested = false;
  }
  
  fn operation_control_3(&mut self, register: u8) {
    if matches!(register & 0b10, 0) {
      debug!("Got an NOP Operation Control Word 3 command: {:X}", register);
      return;
    }
    self.next_get = match register & 0b1 {
      0 => RegisterType::IRR,
      _ => RegisterType::ISR,
    };
    debug!("Got a Operation Control Word 3 to read: {:?}", self.next_get);
  }

  pub(crate) fn interrupt_irq0(&mut self, _: CX![]) {
    if self.irq0.enabled && !self.irq0.interrupted_cpu { //Only PIT channel 0 can interrupt on a x86. Unclear about other machines.
      let interrupt_index = self.vector_offset + 0;
      debug!("PIC IRQ0 triggered! This maps to INT {}", interrupt_index);
      call!([self.board], pic_interrupt(interrupt_index));
      self.irq0.interrupted_cpu = true;
      self.irq0.interrupt_requested = false;
    } else {
      self.irq0.interrupt_requested = true;
    }
  }
}
