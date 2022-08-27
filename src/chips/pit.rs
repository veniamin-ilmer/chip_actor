use stakker::*;
use crate::Board;

use log::debug;

#[derive(Debug, Default)]
enum Mode {
  Interrupt,
  OneShot,
  RateGenerator,
  #[default]
  SquareWave,
  SoftwareStrobe,
  HardwareStrobe,
}

pub(crate) struct PIT {
  c0: ActorOwn<Counter>,
  c1: ActorOwn<Counter>,
  c2: ActorOwn<Counter>
}

impl PIT {
  pub(crate) fn init(cx: CX![], board: Actor<Board>) -> Option<Self> {
    let c0 = actor!(cx, Counter::init(board.clone(), 0), ret_nop!());
    let c1 = actor!(cx, Counter::init(board.clone(), 1), ret_nop!());
    let c2 = actor!(cx, Counter::init(board.clone(), 2), ret_nop!());

    Some(PIT {
      c0, c1, c2
    })
  }

  pub(crate) fn set_count0(&self, _: CX![], value: u8) {
    call!([self.c0], set_count(value));
  }
  pub(crate) fn set_count1(&self, _: CX![], value: u8) {
    call!([self.c1], set_count(value));
  }
  pub(crate) fn set_count2(&self, _: CX![], value: u8) {
    call!([self.c2], set_count(value));
  }

  pub(crate) fn get_count0(&self, _: CX![], ret: Ret<u8>) {
    call!([self.c0], get_count(ret));
  }
  pub(crate) fn get_count1(&self, _: CX![], ret: Ret<u8>) {
    call!([self.c1], get_count(ret));
  }
  pub(crate) fn get_count2(&self, _: CX![], ret: Ret<u8>) {
    call!([self.c2], get_count(ret));
  }

  pub(crate) fn set_control_word(&self, _: CX![], value: u8) {
    let select_counter = (value & 0b1100_0000) >> 6;
    let counter = match select_counter {
      0 => &self.c0,
      1 => &self.c1,
      2 => &self.c2,
      3 => unimplemented!(),  //TODO 8254 function "Read-back command"
      _ => unreachable!(),
    };
    call!([counter], set_control_word(value));
  }
}

#[derive(Debug, Default)]
enum Access {
  #[default]
  LSB,
  MSB,
  LSBThenMSB,
}

pub(crate) struct Counter {
  board: Actor<crate::Board>,
  select_counter: u8,
  enabled: bool,
  latched: bool,
  initial_count_register: u16,
  counting_element: u16,
  output_latch: u16,
  mode: Mode,
  access: Access,
  low_count: Option<u16>,       //This should only be set with set_count(..) flip_flop Low.
}

impl Counter {
  
  pub(crate) fn init(_: CX![], board: Actor<Board>, select_counter: u8) -> Option<Self> {
    Some(Counter {
      board,
      select_counter,
      enabled: true,
      latched: false,
      initial_count_register: 0,
      counting_element: 0,
      output_latch: 0,
      mode: Default::default(),
      access: Default::default(),
      low_count: None,
    })
  }
  
  fn single_run(&mut self, cx: CX![]) {
    if !self.enabled {
      return;
    }
    self.counting_element = self.counting_element.wrapping_sub(1);
    if self.counting_element == 0 {
      if let Mode::Interrupt = self.mode {
        self.enabled = false;
        debug!("Interrupting PIC");
        call!([self.board], timer_interrupt(self.select_counter));
      } else {
        //Starting over again
        self.counting_element = self.initial_count_register;
      }
    }
    if !self.latched {
      self.output_latch = self.counting_element;
    }
    
    //Check if enabled because we might have disabled it due to an interrupt
    if self.enabled {
      let duration = std::time::Duration::from_nanos(838); //1.193182 Mhz = 838 nanosecond delay.
      let instant = cx.now() + duration;
      at!(instant, [cx], single_run());
    }
  }
  
  fn set_control_word(&mut self, _: CX![], value: u8) {
    if (value & 0b11_0000) >> 4 == 0 {  //Latch mode
      self.latched = true;
    } else {  //Initialization mode
      //Hopefully I won't have to build this..
      if matches!(value & 0b1, 0b1) {
        unimplemented!("BCD requested for PIT..");
      }
      
      self.mode = match (value & 0b1110) >> 1 {
        0 => Mode::Interrupt,
        1 => Mode::OneShot,
        2 | 6 => Mode::RateGenerator,
        3 | 7 => Mode::SquareWave,
        4 => Mode::SoftwareStrobe,
        5 | _ => Mode::HardwareStrobe,
      };
      
      self.access = match (value & 0b11_0000) >> 4 {
        1 => Access::LSB,
        2 => Access::MSB,
        3 => {self.low_count = None; Access::LSBThenMSB},
        _ => unreachable!(),
      };

      debug!("Counter {}: mode: {:?}, access: {:?}", self.select_counter, self.mode, self.access);
    }
  }
  
  /// Datasheet: if the Counter has been programmed for one
  /// byte counts (either most significant byte only or least
  /// significant byte only) the other byte will be zero.
  fn set_count(&mut self, cx: CX![], value: u8) {
    let new_count = value as u16;
    let count_register: Option<u16> = match self.access {
      Access::LSB => Some(new_count),
      Access::MSB => Some(new_count << 8),
      Access::LSBThenMSB => match self.low_count {
        None => {
          self.low_count = Some(new_count);
          None  //Don't trigger yet. Wait for the next value to be given first.
        },
        Some(low) => {
          self.low_count = None;
          Some(low + (new_count << 8))
        },
      },
    };
    if let Some(count) = count_register {
      self.initial_count_register = count;
      self.counting_element = count;
      debug!("Counter {}'s count_register was set to {:X}", self.select_counter, count);
      self.enabled = true;
      call!([cx], single_run());
    } else if let Some(count) = self.low_count {
      debug!("Counter {}'s was given a low count {:X}", self.select_counter, count);
    }
  }

  fn get_count(&mut self, cx: CX![], ret: Ret<u8>) {
    let mut release_latch = true;
    let output_latch = self.output_latch;
    let count_u8 = {
      match self.access {
        Access::LSB => output_latch & 0xFF,
        Access::MSB => output_latch >> 8,
        Access::LSBThenMSB => match self.low_count {
          None => {
            {
              release_latch = false;  //Don't release the latch yet, since we are only reading the low byte.
              self.low_count = Some(0);
              output_latch & 0xFF
            }
          },
          Some(_) => {
            self.low_count = None;
            output_latch >> 8
          }
        },
      }
    } as u8;
    if release_latch {
      self.latched = false;
    }
    
    debug!("Read Counter {}'s count {:X}", self.select_counter, count_u8);
    ret!([ret], count_u8);
  }
}