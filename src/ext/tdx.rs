//! tdx ext dx bridge for DxLib
//!

use std::ffi::c_void;
use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::{dx::*, ext::*};
use crate::ext::music::Music;
use crate::ext::sound::Sound;
use crate::ext::graph::Graph;

pub struct Tdx {
  pub tbl: HashMap<i32, Rc<RefCell<Box<dyn Tr>>>>
}

impl Tdx {
  pub fn new() -> Result<Self, Box<dyn Error>> {
    if unsafe { DxLib_Init() } == -1 { return Err("Cannot init DxLib".into()) }
    Ok(Tdx{tbl: HashMap::new()})
  }

  pub fn reg(&mut self, o: impl Tr + 'static) -> Rc<RefCell<Box<dyn Tr>>> {
    let h = o.handle();
    self.tbl.insert(h, Rc::new(RefCell::new(Box::new(o))));
    self.tbl.get(&h).expect("get").clone()
  }

  pub fn load_music_mem(&mut self, n: &String) -> Rc<RefCell<Box<dyn Tr>>> {
    self.reg(Music::load_mem(n))
  }

  pub fn load_sound_mem(&mut self, n: &String) -> Rc<RefCell<Box<dyn Tr>>> {
    self.reg(Sound::load_mem(n))
  }

  pub fn load_graph(&mut self, n: &String) -> Rc<RefCell<Box<dyn Tr>>> {
    self.reg(Graph::load(n))
  }

  pub fn get_color(r: i32, g: i32, b: i32) -> u32 {
    unsafe { GetColor(r, g, b) }
  }

  pub fn draw_pixel(x: i32, y: i32, c: u32) -> i32 {
    unsafe { DrawPixel(x, y, c) }
  }

  pub fn process_message() -> i32 {
    unsafe { ProcessMessage() }
  }

  pub fn screen_flip() -> i32 {
    unsafe { ScreenFlip() }
  }

  pub fn clear_draw_screen(r: *const c_void) -> i32 {
    unsafe { ClearDrawScreen(r) }
  }

  pub fn set_draw_screen(s: i32) -> i32 {
    unsafe { SetDrawScreen(s) }
  }

  pub fn init_font_to_handle() -> i32 {
    unsafe { InitFontToHandle() }
  }

  pub fn init_shader() -> i32 {
    unsafe { InitShader() }
  }

  pub fn process_music_mem() -> i32 {
    unsafe { ProcessMusicMem() }
  }

  /// DX_MIDIMODE_DM DX_MIDIMODE_MCI (default)
  pub fn select_midi_mode(m: i32) -> i32 {
    unsafe { SelectMidiMode(m) }
  }

  pub fn init_music_mem() -> i32 {
    unsafe { InitMusicMem() }
  }

  pub fn wait_key() -> i32 {
    unsafe { WaitKey() }
  }

  pub fn wait_timer(ms: i32) -> i32 {
    unsafe { WaitTimer(ms) }
  }

  pub fn set_main_window_text(t: &str) {
    unsafe { SetMainWindowText(t.as_ptr()); }
  }

  pub fn set_main_window_text_bytes(t: &[u8]) {
    unsafe { SetMainWindowText(t.as_ptr()); }
  }

  pub fn set_graph_mode(w: i32, h: i32, b: i32, fps: i32) {
    unsafe { SetGraphMode(w, h, b, fps); }
  }

  pub fn change_window_mode(f: i32) {
    unsafe { ChangeWindowMode(f); }
  }

  pub fn set_out_application_log_valid_flag(f: i32) {
    unsafe { SetOutApplicationLogValidFlag(f); }
  }
}

impl Drop for Tdx {
  fn drop(&mut self) {
    for (_k, v) in self.tbl.iter_mut() { v.dispose(); }
    unsafe { DxLib_End(); }
  }
}
