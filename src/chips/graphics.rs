use stakker::*;

//6845 - Motorola CRT Controller
//https://stanislavs.org/helppc/6845.html

use log::{debug, error};

#[derive(Debug, Default)]
enum TextSize {
  #[default]
  D40x25, //Smaller text
  D80x25, //Bigger text
}

#[derive(Debug, Default)]
enum GraphicsType {
  #[default]
  D320x200,
  Text,
}

#[derive(Debug, Default)]
pub(crate) struct BWOptions {
  text_size: TextSize,
  enabled: bool,
  blink: bool,
}

#[derive(Debug, Default)]
pub(crate) struct ColorOptions {
  graphics_type: GraphicsType,
  black_white: bool,
  black_white_640x200: bool,
}

#[derive(Debug, Default)]
enum Register {
  #[default]
  HorizontalTotalCharacter,
  HorizontalDisplayedCharactersPerLine,
  HorizontalSyncPosition,
  HorizontalSyncCharacterWidth,
  VerticalTotalLines,
  VerticalTotalAdjust,
  VerticalDisplayedRows,
  VerticalSyncCharacterRows,
  InterlaceMode,
  MaximumScanLineAddress,
  CursorStart,
  CursorEnd,
  StartAddressMSB,
  StartAddressLSB,
  CursorAddressMSB,
  CursorAddressLSB,
  LightPenMSB,
  LightPenLSB,
}

#[derive(Debug, Default)]
pub(crate) struct Graphics {
  bw_options: BWOptions,
  color_options: ColorOptions,
  current_register: Register,
  horizontal_total_character: u8,
  horizontal_displayed_characters_per_line: u8,
  horizontal_sync_position: u8,
  horizontal_sync_character_width: u8,
  vertical_total_lines: u8,
  vertical_total_adjust: u8,
  vertical_displayed_rows: u8,
  vertical_sync_character_rows: u8,
  interlace_mode: u8,
  maximum_scan_line_address: u8,
  cursor_start: u8,
  cursor_end: u8,
  start_address: u16,
  cursor_address: u16,
  light_pen: u16,
}

impl Graphics {
  pub(crate) fn init(_: CX![]) -> Option<Graphics> {
    Some(Default::default())
  }

  pub(crate) fn set_mode_bw(&mut self, _:CX![], register: u8) {
    self.bw_options.text_size = if matches!(register & 0b1, 0b1) { TextSize::D80x25 } else { TextSize::D40x25};
    self.bw_options.enabled = matches!(register & 0b1000, 0b1000);
    self.bw_options.blink = matches!(register & 0b10_0000, 0b10_0000);
    debug!("{:?}", self.bw_options);
  }
  
  pub(crate) fn get_mode_bw(&mut self) -> u8 {
    let mut result = 0;
    result |= match self.bw_options.text_size {
      TextSize::D40x25 => 0,
      TextSize::D80x25 => 1,
    };
    if self.bw_options.enabled {
      result |= 0b1000;
    }
    if self.bw_options.blink {
      result |= 0b10_0000;
    }
    debug!("{:?}", self.color_options);
    result
  }
  
  pub(crate) fn set_mode_color(&mut self, cx:CX![], register: u8) {
    self.set_mode_bw(cx, register);
    self.color_options.graphics_type = if matches!(register & 0b10, 0b10) { GraphicsType::D320x200 } else { GraphicsType::Text };
    self.color_options.black_white = matches!(register & 0b100, 0b100);
    self.color_options.black_white_640x200 = matches!(register & 0b1_0000, 0b1_0000);
    debug!("{:?}", self.color_options);
  }
  
  pub(crate) fn choose_register(&mut self, register: u8) {
    self.current_register = match register {
      0x00 => Register::HorizontalTotalCharacter,
      0x01 => Register::HorizontalDisplayedCharactersPerLine,
      0x02 => Register::HorizontalSyncPosition,
      0x03 => Register::HorizontalSyncCharacterWidth,
      0x04 => Register::VerticalTotalLines,
      0x05 => Register::VerticalTotalAdjust,
      0x06 => Register::VerticalDisplayedRows,
      0x07 => Register::VerticalSyncCharacterRows,
      0x08 => Register::InterlaceMode,
      0x09 => Register::MaximumScanLineAddress,
      0x0A => Register::CursorStart,
      0x0B => Register::CursorEnd,
      0x0C => Register::StartAddressMSB,
      0x0D => Register::StartAddressLSB,
      0x0E => Register::CursorAddressMSB,
      0x0F => Register::CursorAddressLSB,
      0x10 => Register::LightPenMSB,
      0x11 => Register::LightPenLSB,
      _ => {error!("Invalid Register index passed to graphics card: {}", register); Register::HorizontalTotalCharacter},
    };
    debug!("Referencing {:?}", self.current_register);
  }
  
  pub(crate) fn set_register_data(&mut self, register: u8) {
    match self.current_register {
      Register::HorizontalTotalCharacter => self.horizontal_total_character = register,
      Register::HorizontalDisplayedCharactersPerLine => self.horizontal_displayed_characters_per_line = register,
      Register::HorizontalSyncPosition => self.horizontal_sync_position = register,
      Register::HorizontalSyncCharacterWidth => self.horizontal_sync_character_width = register,
      Register::VerticalTotalLines => self.vertical_total_lines = register,
      Register::VerticalTotalAdjust => self.vertical_total_adjust = register,
      Register::VerticalDisplayedRows => self.vertical_displayed_rows = register,
      Register::VerticalSyncCharacterRows => self.vertical_sync_character_rows = register,
      Register::InterlaceMode => self.interlace_mode = register,
      Register::MaximumScanLineAddress => self.maximum_scan_line_address = register,
      Register::CursorStart => self.cursor_start = register,
      Register::CursorEnd => self.cursor_end = register,
      Register::StartAddressMSB => self.start_address = (self.start_address & 0xFF) | ((register as u16) << 8),
      Register::StartAddressLSB => self.start_address = (self.start_address & 0xFF00) | (register as u16),
      Register::CursorAddressMSB => self.cursor_address = (self.cursor_address & 0xFF) | ((register as u16) << 8),
      Register::CursorAddressLSB => self.cursor_address = (self.cursor_address & 0xFF00) | (register as u16),
      Register::LightPenMSB => self.light_pen = (self.light_pen & 0xFF) | ((register as u16) << 8),
      Register::LightPenLSB => self.light_pen = (self.light_pen & 0xFF00) | (register as u16),
    }
    debug!("Set value {:X}", register);
  }
}
