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
type PIC = chips::pic::PIC;
type FixedDisk = chips::fixed_disk::FixedDisk;


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
  
  let video_rom = Vec::new();
  /*
  f = File::open("roms/ibm-ega-1984-09-13.rom").unwrap();
  let mut video_rom = Vec::new();
  f.read_to_end(&mut video_rom).unwrap();
  */
  f = File::open("roms/ibm-mfm-1985-10-28.rom").unwrap();
  let mut disk_rom = Vec::new();
  f.read_to_end(&mut disk_rom).unwrap();
  
  let board = actor_new!(stakker, Board, ret_nop!());
  let cpu = actor!(stakker, CPU::init(board.clone(), bios_rom, video_rom, disk_rom), ret_nop!());
  let timer = actor!(stakker, Timer::init(board.clone()), ret_nop!());
  let ppi = actor!(stakker, PPI::init(), ret_nop!());
  let graphics = actor!(stakker, Graphics::init(), ret_nop!());
  let memory_controller = actor!(stakker, MemoryController::init(), ret_nop!());
  let pic = actor!(stakker, PIC::init(board.clone()), ret_nop!());
  let fixed_disk = actor!(stakker, FixedDisk::init(board.clone()), ret_nop!());
  call!([board], Board::init(
    cpu.clone(),
    timer.clone(),
    ppi.clone(),
    graphics.clone(),
    memory_controller.clone(),
    pic.clone(),
    fixed_disk.clone()
  ));
  
  call!([cpu], single_run());
  
  while stakker.not_shutdown() {
    // Run queue and timers
    stakker.run(time::Instant::now(), false);
  }
}
