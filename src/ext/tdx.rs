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
use crate::ext::shader::{VertexShader, PixelShader, GeometryShader};
use crate::ext::font::Font;

pub struct Tdx {
  pub tbl: HashMap<i32, RcTr>
}

impl Tdx {
  pub fn new() -> Result<Self, Box<dyn Error>> {
    if unsafe { DxLib_Init() } == -1 { return Err("Cannot init DxLib".into()) }
    Ok(Tdx{tbl: HashMap::new()})
  }

  pub fn reg(&mut self, o: Box<dyn Tr>) -> RcTr {
    let h = o.handle();
    self.tbl.insert(h, Rc::new(RefCell::new(o)));
    self.tbl.get(&h).expect("get").clone()
  }

  pub fn load_music_mem(&mut self, n: &String) -> RcTr {
    self.reg(Box::new(Music::load_mem(n)))
  }

  pub fn load_sound_mem(&mut self, n: &String) -> RcTr {
    self.reg(Box::new(Sound::load_mem(n)))
  }

  pub fn load_graph(&mut self, n: &String) -> RcTr {
    self.reg(Box::new(Graph::load(n)))
  }

  pub fn load_vertex_shader(&mut self, n: &String) -> RcTr {
    self.reg(Box::new(VertexShader::load(n)))
  }

  pub fn load_pixel_shader(&mut self, n: &String) -> RcTr {
    self.reg(Box::new(PixelShader::load(n)))
  }

  pub fn load_geometry_shader(&mut self, n: &String) -> RcTr {
    self.reg(Box::new(GeometryShader::load(n)))
  }

  pub fn create_font(&mut self, n: &str, sz: i32, thick: i32,
    fonttype: i32, charset: i32, edgesz: i32, italic: i32) -> RcTr {
    self.reg(
      Box::new(Font::create(n, sz, thick, fonttype, charset, edgesz, italic)))
  }

  pub fn load_font(&mut self, n: &String) -> RcTr {
    self.reg(Box::new(Font::load_data(n)))
  }
}

impl Drop for Tdx {
  fn drop(&mut self) {
    for (_k, v) in self.tbl.iter_mut() { v.dispose(); }
    unsafe { DxLib_End(); }
  }
}

pub fn change_window_mode(f: i32) -> i32 {
  unsafe { ChangeWindowMode(f) }
}

pub fn set_graph_mode(w: i32, h: i32, b: i32, fps: i32) -> i32 {
  unsafe { SetGraphMode(w, h, b, fps) }
}

pub fn set_out_application_log_valid_flag(f: i32) -> i32 {
  unsafe { SetOutApplicationLogValidFlag(f) }
}

pub fn set_main_window_text(t: &str) -> i32 {
  unsafe { SetMainWindowText(t.as_ptr()) }
}

pub fn set_main_window_text_bytes(t: &[u8]) -> i32 {
  unsafe { SetMainWindowText(t.as_ptr()) }
}

pub fn set_draw_blend_mode(bm: i32, pal: i32) -> i32 {
  unsafe { SetDrawBlendMode(bm, pal) }
}

pub fn get_joypad_input_state(inputtype: i32) -> i32 {
  unsafe { GetJoypadInputState(inputtype) }
}

pub fn get_hit_key_state_all(ksbuf: *const u8) -> i32 {
  unsafe { GetHitKeyStateAll(ksbuf) }
}

pub fn wait_key() -> i32 {
  unsafe { WaitKey() }
}

pub fn wait_timer(ms: i32) -> i32 {
  unsafe { WaitTimer(ms) }
}

pub fn process_message() -> i32 {
  unsafe { ProcessMessage() }
}

pub fn clear_draw_screen(r: *const c_void) -> i32 {
  unsafe { ClearDrawScreen(r) }
}

pub fn set_draw_screen(s: i32) -> i32 {
  unsafe { SetDrawScreen(s) }
}

pub fn screen_flip() -> i32 {
  unsafe { ScreenFlip() }
}

/// DX_MIDIMODE_DM DX_MIDIMODE_MCI (default)
pub fn select_midi_mode(m: i32) -> i32 {
  unsafe { SelectMidiMode(m) }
}

pub fn init_music_mem() -> i32 {
  unsafe { InitMusicMem() }
}

pub fn process_music_mem() -> i32 {
  unsafe { ProcessMusicMem() }
}

pub fn init_shader() -> i32 {
  unsafe { InitShader() }
}

pub fn set_use_back_culling(f: i32) -> i32 {
  unsafe { SetUseBackCulling(f) }
}

pub fn create_look_at_matrix(o: &mut MATRIX,
  eye: &VECTOR, at: &VECTOR, up: &VECTOR) -> i32 {
  unsafe { CreateLookAtMatrix(o as *mut MATRIX,
    eye as *const VECTOR, at as *const VECTOR, up as *const VECTOR) }
}

pub fn set_camera_near_far(near: f32, far: f32) -> i32 {
  unsafe { SetCameraNearFar(near, far) }
}

/// (move) MTranspose (GL &lt; - &gt; DX)
pub fn set_camera_view_matrix(vm: MATRIX) -> i32 {
  unsafe { SetCameraViewMatrix(vm) }
}

pub fn get_camera_projection_matrix() -> MATRIX {
  unsafe { GetCameraProjectionMatrix() }
}

pub fn get_projection_matrix() -> MATRIX {
  unsafe { GetProjectionMatrix() }
}

pub fn set_transform_to_projection(m: &MATRIX) -> i32 {
  unsafe { SetTransformToProjection(m as *const MATRIX) }
}

/// default aspect -1.0
pub fn create_perspective_fov_matrix(m: &mut MATRIX,
  fov: f32, zn: f32, zf: f32, aspect: f32) -> i32 {
  unsafe { CreatePerspectiveFovMatrix(m as *mut MATRIX, fov, zn, zf, aspect) }
}

pub fn create_viewport_matrix(m: &mut MATRIX,
  cx: f32, cy: f32, w: f32, h: f32) -> i32 {
  unsafe { CreateViewportMatrix(m as *mut MATRIX, cx, cy, w, h) }
}

pub fn set_transform_to_viewport(m: &MATRIX) -> i32 {
  unsafe { SetTransformToViewport(m as *const MATRIX) }
}

pub fn draw_polygon_3d_to_shader(va: &VERTEX3DSHADER, npolygons: i32) -> i32 {
  unsafe { DrawPolygon3DToShader(va as *const VERTEX3DSHADER, npolygons) }
}

pub fn init_font_to_handle() -> i32 {
  unsafe { InitFontToHandle() }
}

pub fn get_color(r: i32, g: i32, b: i32) -> u32 {
  unsafe { GetColor(r, g, b) }
}

pub fn draw_pixel(x: i32, y: i32, c: u32) -> i32 {
  unsafe { DrawPixel(x, y, c) }
}
