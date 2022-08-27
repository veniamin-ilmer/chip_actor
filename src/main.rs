use stakker::*;

use std::io::prelude::*;
use std::fs::File;

use std::time;
mod chips;
mod motherboards;

type Board = motherboards::ibm_xt::IBM_XT;
type CPU = chips::cpu8088::CPU;
type Timer = chips::pit::PIT;
type PPI = chips::ppi::PPI;
type Graphics = chips::graphics::Graphics;
type MemoryController = chips::dma::DMA;

use simplelog::*;

fn main() {
  TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();
//  TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();
/*
  CombinedLogger::init(vec![
      TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
      WriteLogger::new(LevelFilter::Trace, Config::default(), File::create("trace.log").unwrap()),
  ]).unwrap();
*/

  let mut stakker0 = Stakker::new(time::Instant::now());
  let stakker = &mut stakker0;
  
  let mut f = File::open("roms/ibm-xt-1986-05-09.rom").unwrap();
  let mut bios_rom = Vec::new();
  f.read_to_end(&mut bios_rom).unwrap();
  
//  f = File::open("roms/ibm-mfm-1985-10-28.rom")?;
//  let mut video_rom = Vec::new();
//  f.read_to_end(&mut video_rom)?;
  let video_rom = Vec::new();
  
  let board = actor_new!(stakker, Board, ret_nop!());
  let cpu = actor!(stakker, CPU::init(board.clone(), bios_rom, video_rom), ret_nop!());
  let timer = actor!(stakker, Timer::init(board.clone()), ret_nop!());
  let ppi = actor!(stakker, PPI::init(), ret_nop!());
  let graphics = actor!(stakker, Graphics::init(), ret_nop!());
  let memory_controller = actor!(stakker, MemoryController::init(), ret_nop!());
  call!([board], Board::init(
    cpu.clone(),
    timer.clone(),
    ppi.clone(),
    graphics.clone(),
    memory_controller.clone()
  ));
  
  call!([cpu], single_run());
  
  while stakker.not_shutdown() {
    // Run queue and timers
    stakker.run(time::Instant::now(), false);
  }
}
