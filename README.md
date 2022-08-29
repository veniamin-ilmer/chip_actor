# Chip Actor

## Fully emulated chips

* Intel 8088 - CPU
* Intel 8259 - Programmable Interrupt Controller (PIC)
* Intel 8253 - Programmable Interval Timer (PIT)

## Partially emulated chips

* Intel 8237 / AMD 1980 - Direct Memory Access (DMA)
* Intel 8255 - Peripheral  Programmable Peripheral Interface (PPI)
* Motorola 6845 - CRT Controller
* IBM XT Hard Disk Controller

## About

The design of this project has went through multiple revisions.

I originally wanted to build an 8088 emulator which didn't "cheat" with simulating dos/bios interrupts, and instead simulate all of the chips.

I found out, building all of the other chips, and designing a way for all of them to communicate with each other, was quite challenging.

I came up with the idea that the CPU shouldn't be treated in a special way. Instead, the CPU is just another chip. All chips are treated equally and have the same capabilities. I also wanted to design the chips to allow for them to be replaceable - they should communicate via the motherboard instead of directly with each other.

At first I had all chips running in a separate threads, communicating via message passing.

After some time however, I realized that the nanosecond time requirements for CPU emulation, requires response times faster than what threaded message passing could deliver.

I rewrote the emulator to run single threaded with cooperative multitasking. Doing this, I preserved the ability for all chips to run "at the same time", and for the CPU to continue to be abstracted as "just another chip". All chips are truly emulated: The timer chip runs at 1.2 MHz, independently of the 8088 CPU running at 4.77 MHz. The main limitation of my design, is that one CPU core is permanently used at 100% due to constantly spinning, scheduling chips to run at nanosecond speeds.

I converted all chips to run on an Actor Model. They are now in sync and running fast and efficiently, with zero message blocking.

I intend to make this emulator into a more general "chip emulator", which could run any system. 8085, z80, etc, along with all peripheral chips. Perhaps I could eventually make a Frankenstein machine composed of chips stitched in from completely different origins.

I also intend this project to be used as a reference. The chip functionality should be fully explained. Perhaps make this into a library for anyone to be able to use any chip.

## Latest Progress

### 2022-08-29

After switching to an Actor messaging model, the BIOS now gets through timing the PIT successfully. It consistently outputs "064 KB OK" into the teletype output.

Next steps: Expand on the PPI. Get it to signal to the BIOS that a floppy drive exists. Figure out how to make the BIOS know that more than 64 KB of memory is available... See what other fun dip switches the PPI has to offer.
