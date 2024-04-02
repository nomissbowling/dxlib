//! tdx ext dx bridge for DxLib
//!

use std::ffi::c_void;
use std::error::Error;
use std::sync::Arc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::{dx::*, ext::*};
use crate::ext::music::Music;
use crate::ext::sound::Sound;
use crate::ext::graph::{Screen, Graph};
use crate::ext::shader::{VertexShader, PixelShader, GeometryShader};
use crate::ext::font::Font;

pub type RcTr = Arc<RefCell<Box<dyn Tr>>>;

pub trait Tr {
  fn as_music(&self) -> Music { panic!("music") }
  fn as_sound(&self) -> Sound { panic!("sound") }
  fn as_screen(&self) -> Screen { panic!("screen") }
  fn as_graph(&self) -> Graph { panic!("graph") }
  fn as_vertex_shader(&self) -> VertexShader { panic!("vertex_shader") }
  fn as_pixel_shader(&self) -> PixelShader { panic!("pixel_shader") }
  fn as_geometry_shader(&self) -> GeometryShader { panic!("geometry_shader") }
  fn as_font(&self) -> Font { panic!("font") }

  fn handle(&self) -> i32;
  fn dispose(&mut self);
}

pub struct Tdx {
  pub tbl: HashMap<i32, RcTr>
}

impl Tdx {
  pub fn new() -> Result<Self, Box<dyn Error>> {
    if unsafe { DxLib_Init() } == -1 { return Err("Cannot init DxLib".into()) }
    Ok(Tdx{tbl: HashMap::new()})
  }

  pub fn unreg(&mut self, o: Box<dyn Tr>) {
    match self.tbl.remove(&o.handle()) {
    None => (), // or expect("unreg dup")
    Some(v) => v.borrow_mut().dispose()
    }
  }

  pub fn reg(&mut self, o: Box<dyn Tr>) -> RcTr {
    let h = o.handle();
    self.tbl.insert(h, Arc::new(RefCell::new(o)));
    self.tbl.get(&h).expect("get").clone()
  }

  pub fn load_music_mem(&mut self, n: &String) -> Music {
    self.reg(Box::new(Music::load_mem(n)))
    .borrow().as_music()
  }

  pub fn load_sound_mem(&mut self, n: &String) -> Sound {
    self.reg(Box::new(Sound::load_mem(n)))
    .borrow().as_sound()
  }

  pub fn make_screen(&mut self, xsz: i32, ysz: i32, trans: i32) -> Screen {
    self.reg(Box::new(Screen::make(xsz, ysz, trans)))
    .borrow().as_screen()
  }

  /// inner change draw screen
  pub fn make_graph_color(&mut self, xsz: i32, ysz: i32, c: u32,
    trans: i32, use_client_flag: i32, not_use_3d_flag: i32) -> Graph {
    let screen = self.make_screen(xsz, ysz, trans);
    screen.set_draw();
    draw_box(0, 0, xsz, ysz, c, TRUE);
    let g = self.get_graph(0, 0, xsz, ysz, use_client_flag, not_use_3d_flag);
    self.unreg(Box::new(screen));
    g
  }

  /// inner change draw screen
  pub fn make_graphs_from_div_graph(&mut self, vg: &Vec<Graph>,
    trans: i32, use_client_flag: i32, not_use_3d_flag: i32) -> Vec<Graph> {
    if vg.len() == 0 { return vec![] }
    let (w, h) = vg[0].get_size();
    let screen = self.make_screen(w, h, trans);
    screen.set_draw();
    let v = vg.iter().map(|src| {
      src.draw(0, 0, trans);
      self.get_graph(0, 0, w, h, use_client_flag, not_use_3d_flag)
    }).collect();
    self.unreg(Box::new(screen));
    v
  }

  pub fn get_graph(&mut self, l: i32, t: i32, w: i32, h: i32,
    use_client_flag: i32, not_use_3d_flag: i32) -> Graph {
    let g = self.reg(Box::new(Graph::make(w, h, not_use_3d_flag)))
    .borrow().as_graph();
    g.get_draw_screen(l, t, l + w, t + h, use_client_flag);
    g
  }

  pub fn make_graph(&mut self, xsz: i32, ysz: i32,
    not_use_3d_flag: i32) -> Graph {
    self.reg(Box::new(Graph::make(xsz, ysz, not_use_3d_flag)))
    .borrow().as_graph()
  }

  pub fn load_graph(&mut self, n: &String) -> Graph {
    self.reg(Box::new(Graph::load(n)))
    .borrow().as_graph()
  }

  pub fn load_div_graph(&mut self, n: &String, allnum: i32,
    xnum: i32, ynum: i32, xsz: i32, ysz: i32,
    not_use_3d_flag: i32, xstride: i32, ystride: i32) -> Vec<Graph> {
    let mut handle_buf = vec![0i32; allnum as usize];
    unsafe { LoadDivGraph(n.as_ptr(), allnum,
      xnum, ynum, xsz, ysz, &mut handle_buf[0] as *mut i32,
      not_use_3d_flag, xstride, ystride); }
    handle_buf.into_iter().map(|h|
      self.reg(Box::new(Graph{d: true, h}))
      .borrow().as_graph()).collect()
  }

  pub fn load_vertex_shader(&mut self, n: &String) -> VertexShader {
    self.reg(Box::new(VertexShader::load(n)))
    .borrow().as_vertex_shader()
  }

  pub fn load_pixel_shader(&mut self, n: &String) -> PixelShader {
    self.reg(Box::new(PixelShader::load(n)))
    .borrow().as_pixel_shader()
  }

  pub fn load_geometry_shader(&mut self, n: &String) -> GeometryShader {
    self.reg(Box::new(GeometryShader::load(n)))
    .borrow().as_geometry_shader()
  }

  pub fn create_font(&mut self, n: &str, sz: i32, thick: i32,
    fonttype: i32, charset: i32, edgesz: i32, italic: i32) -> Font {
    self.reg(
      Box::new(Font::create(n, sz, thick, fonttype, charset, edgesz, italic)))
    .borrow().as_font()
  }

  pub fn load_font(&mut self, n: &String) -> Font {
    self.reg(Box::new(Font::load_data(n)))
    .borrow().as_font()
  }
}

impl Drop for Tdx {
  fn drop(&mut self) {
    for (_k, v) in self.tbl.iter_mut() { v.borrow_mut().dispose(); }
    unsafe { DxLib_End(); }
  }
}

/// flg=TRUE
pub fn set_use_normal_draw_shader(flg: i32) -> i32 {
  unsafe { SetUseNormalDrawShader(flg) }
}

/// flg=FALSE
pub fn set_use_software_render_mode_flag(flg: i32) -> i32 {
  unsafe { SetUseSoftwareRenderModeFlag(flg) }
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

/// DX_BLENDMODE_NOBLEND DX_BLENDMODE_ALPHA DX_BLENDMODE_INVSRC etc
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

pub fn set_material_use_vert_dif_color(flg: i32) -> i32 {
  unsafe { SetMaterialUseVertDifColor(flg) }
}

pub fn set_material_use_vert_spc_color(flg: i32) -> i32 {
  unsafe { SetMaterialUseVertSpcColor(flg) }
}

/// (move)
pub fn set_material_param(mp: MATERIALPARAM) -> i32 {
  unsafe { SetMaterialParam(mp) }
}

pub fn set_use_lighting(flg: i32) -> i32 {
  unsafe { SetUseLighting(flg) }
}

pub fn set_use_back_culling(f: i32) -> i32 {
  unsafe { SetUseBackCulling(f) }
}

/// surface_index=0, mip_level=0
pub fn set_render_target_to_shader(target_index: i32, draw_screen: i32,
  surface_index: i32, mip_level: i32) -> i32 {
  unsafe {
    SetRenderTargetToShader(target_index, draw_screen,
      surface_index, mip_level)
  }
}

/// flg=FALSE (2D 3D)
pub fn set_use_z_buffer_flag(flg: i32) -> i32 {
  unsafe { SetUseZBufferFlag(flg) }
}

/// flg=FALSE (2D 3D)
pub fn set_write_z_buffer_flag(flg: i32) -> i32 {
  unsafe { SetWriteZBufferFlag(flg) }
}

/// flg=FALSE (3D)
pub fn set_use_z_buffer_3d(flg: i32) -> i32 {
  unsafe { SetUseZBuffer3D(flg) }
}

/// flg=FALSE (3D)
pub fn set_write_z_buffer_3d(flg: i32) -> i32 {
  unsafe { SetWriteZBuffer3D(flg) }
}

/// z=0.2 (2D)
pub fn set_draw_z(z: f32) -> i32 {
  unsafe { SetDrawZ(z) }
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

pub fn get_transform_to_projection_matrix(m: &mut MATRIX) -> i32 {
  unsafe { GetTransformToProjectionMatrix(m as *mut MATRIX) }
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

pub fn draw_polygon_3d_to_shader(va: &Vec<VERTEX3DSHADER>) -> i32 {
  unsafe {
    DrawPolygon3DToShader(&va[0] as *const VERTEX3DSHADER, va.len() as i32 / 3)
  }
}

pub fn draw_polygon_3d(va: &Vec<VERTEX3D>, gh: i32, trans: i32) -> i32 {
  unsafe {
    DrawPolygon3D(&va[0] as *const VERTEX3D, va.len() as i32 / 3, gh, trans)
  }
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

pub fn draw_box(l: i32, t: i32, r: i32, b: i32, c: u32, fill: i32) -> i32 {
  unsafe { DrawBox(l, t, r, b, c, fill) }
}

pub fn set_window_style_mode(s: i32) -> i32 {
  unsafe { SetWindowStyleMode(s) }
}

pub fn set_use_back_buffer_trans_color_flag(f: i32) -> i32 {
  unsafe { SetUseBackBufferTransColorFlag(f) }
}

pub fn set_use_direct_3d_version(v: i32) -> i32 {
  unsafe { SetUseDirect3DVersion(v) }
}
