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
use crate::ext::shader::ConstantBuffer;
use crate::ext::light::Light;
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
  fn as_constant_buffer(&self) -> ConstantBuffer { panic!("constant_buffer") }
  fn as_light(&self) -> Light { panic!("light") }
  fn as_font(&self) -> Font { panic!("font") }

  fn handle(&self) -> i32;
  fn dispose(&mut self);
}

/// for DX11 and DX9
pub trait Ts: Tr {
  /// for DX11
  fn set_const(&self, cb: &ConstantBuffer) -> i32;
  /// must end 0 (for DX9)
  fn get_const_default_param_f_to_shader(&self, n: &str) -> *const FLOAT4 {
    unsafe { GetConstDefaultParamFToShader(n.as_ptr(), self.handle()) }
  }
  /// with len (for DX9)
  fn get_const_default_param_f_to_shader_with_str_len(&self, n: &str, l: usize) -> *const FLOAT4 {
    unsafe { GetConstDefaultParamFToShaderWithStrLen(n.as_ptr(), l, self.handle()) }
  }
  /// must end 0 (for DX9)
  fn get_const_index_to_shader(&self, n: &str) -> i32 {
    unsafe { GetConstIndexToShader(n.as_ptr(), self.handle()) }
  }
  /// with len (for DX9)
  fn get_const_index_to_shader_with_str_len(&self, n: &str, l: usize) -> i32 {
    unsafe { GetConstIndexToShaderWithStrLen(n.as_ptr(), l, self.handle()) }
  }
  /// must end 0 (for DX9)
  fn get_const_count_to_shader(&self, n: &str) -> i32 {
    unsafe { GetConstCountToShader(n.as_ptr(), self.handle()) }
  }
  /// with len (for DX9)
  fn get_const_count_to_shader_with_str_len(&self, n: &str, l: usize) -> i32 {
    unsafe { GetConstCountToShaderWithStrLen(n.as_ptr(), l, self.handle()) }
  }
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

  /// for DX11
  /// - n: number of FLOAT4 (alloc n * 4 * sizeof f32)
  /// - s: slot
  pub fn create_constant_buffer(&mut self, n: i32, s: i32) -> ConstantBuffer {
    self.reg(Box::new(ConstantBuffer::create(n, s)))
    .borrow().as_constant_buffer()
  }

  /// (move)
  pub fn create_dir_light(&mut self, d: VECTOR) -> Light {
    self.reg(Box::new(Light::create_dir(d)))
    .borrow().as_light()
  }

  /// (move)
  pub fn create_spot_light(&mut self, p: VECTOR, d: VECTOR, oa: f32, ia: f32,
    rng: f32, a0: f32, a1: f32, a2: f32) -> Light {
    self.reg(Box::new(Light::create_spot(p, d, oa, ia, rng, a0, a1, a2)))
    .borrow().as_light()
  }

  /// (move)
  pub fn create_point_light(&mut self, p: VECTOR,
    rng: f32, a0: f32, a1: f32, a2: f32) -> Light {
    self.reg(Box::new(Light::create_point(p, rng, a0, a1, a2)))
    .borrow().as_light()
  }

  pub fn get_enable_light_handle_num(&self) -> i32 {
    unsafe { GetEnableLightHandleNum() }
  }

  pub fn get_enable_light_handle(&self, i: i32) -> i32 {
// TODO: find from tbl and as_light before
    unsafe { GetEnableLightHandle(i) }
  }

  pub fn delete_light_handle_all(&mut self) -> i32 {
// TODO: unreg lights from tbl before
    unsafe { DeleteLightHandleAll() }
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

/// default DX_CHECKINPUT_ALL
pub fn check_hit_key_all(typ: i32) -> i32 {
  unsafe { CheckHitKeyAll(typ) }
}

pub fn check_hit_key(code: i32) -> i32 {
  unsafe { CheckHitKey(code) }
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

pub fn set_light_use_shadow_map_handle(lh: i32, ssi: i32, flg: i32) -> i32 {
  unsafe { SetLightUseShadowMapHandle(lh, ssi, flg) }
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

/// default TRUE
pub fn set_use_lighting(flg: i32) -> i32 {
  unsafe { SetUseLighting(flg) }
}

/// default TRUE
pub fn set_use_specular(flg: i32) -> i32 {
  unsafe { SetUseSpecular(flg) }
}

/// (move)
pub fn set_global_ambient_light(c: COLOR_F) -> i32 {
  unsafe { SetGlobalAmbientLight(c) }
}

/// default TRUE
pub fn set_use_light_angle_attenuation(flg: i32) -> i32 {
  unsafe { SetUseLightAngleAttenuation(flg) }
}

/// default TRUE
pub fn set_light_enable(flg: i32) -> i32 {
  unsafe { SetLightEnable(flg) }
}

/// (move) default 1 1 1 1
pub fn set_light_dif_color(c: COLOR_F) -> i32 {
  unsafe { SetLightDifColor(c) }
}

/// (move) default 1 1 1 1 or .5 .5 .5 .5
pub fn set_light_spc_color(c: COLOR_F) -> i32 {
  unsafe { SetLightSpcColor(c) }
}

/// (move) default .33 .33 .33 .33
pub fn set_light_amb_color(c: COLOR_F) -> i32 {
  unsafe { SetLightAmbColor(c) }
}

/// (move) default 1 -1 1 (variable)
pub fn set_light_direction(d: VECTOR) -> i32 {
  unsafe { SetLightDirection(d) }
}

/// (move) no effect to directional light
pub fn set_light_position(p: VECTOR) -> i32 {
  unsafe { SetLightPosition(p) }
}

/// attenuation distance 100.0 / (a0 + a1 * d + a2 * d * d) when d &lt;= rng
pub fn set_light_range_atten(rng: f32, a0: f32, a1: f32, a2: f32) -> i32 {
  unsafe { SetLightRangeAtten(rng, a0, a1, a2) }
}

/// oa 0-DX_PI_F ia 0-oa
pub fn set_light_angle(oa: f32, ia: f32) -> i32 {
  unsafe { SetLightAngle(oa, ia) }
}

pub fn set_light_use_shadow_map(ssi: i32, flg: i32) -> i32 {
  unsafe { SetLightUseShadowMap(ssi, flg) }
}

/// for DX11
pub fn init_shader_constant_buffer() -> i32 {
  unsafe { InitShaderConstantBuffer() }
}

/// (move) for DX9
pub fn set_vs_const_f(i: i32, p: FLOAT4) -> i32 {
  unsafe { SetVSConstF(i, p) }
}

/// (move) for DX9
pub fn set_ps_const_f(i: i32, p: FLOAT4) -> i32 {
  unsafe { SetPSConstF(i, p) }
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

pub fn draw_polygon_3d_to_shader_or_wire(va: &Vec<VERTEX3DSHADER>,
  wf: bool) -> i32 {
  match wf {
  true => {
    let c = get_color(240, 192, 32);
    for i in 0..(va.len() / 3) {
      let p = (0..3).into_iter().map(|k|
        &va[i * 3 + k].pos).collect::<Vec<_>>();
      draw_triangle_3d(p[0].clone(), p[1].clone(), p[2].clone(), c, 0);
    }
    0
  },
  false => draw_polygon_3d_to_shader(va)
  }
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

/// (move)
pub fn draw_pixel_3d(p: VECTOR, c: u32) -> i32 {
  unsafe { DrawPixel3D(p, c) }
}

/// (move)
pub fn draw_line_3d(s: VECTOR, e: VECTOR, c: u32) -> i32 {
  unsafe { DrawLine3D(s, e, c) }
}

/// (move)
pub fn draw_triangle_3d(p0: VECTOR, p1: VECTOR, p2: VECTOR,
  c: u32, fill: i32) -> i32 {
  unsafe { DrawTriangle3D(p0, p1, p2, c, fill) }
}

/// (move)
pub fn draw_cube_3d(p0: VECTOR, p1: VECTOR,
  dif: u32, spc: u32, fill: i32) -> i32 {
  unsafe { DrawCube3D(p0, p1, dif, spc, fill) }
}

/// cube set as vec
pub fn draw_cube_set_3d(ca: &Vec<CUBEDATA>, fill: i32) -> i32 {
  unsafe { DrawCubeSet3D(&ca[0] as *const CUBEDATA, ca.len() as i32, fill) }
}

/// (move)
pub fn draw_sphere_3d(c: VECTOR, r: f32, div_num: i32,
  dif: u32, spc: u32, fill: i32) -> i32 {
  unsafe { DrawSphere3D(c, r, div_num, dif, spc, fill) }
}

/// (move)
pub fn draw_capsule_3d(p0: VECTOR, p1: VECTOR, r: f32, div_num: i32,
  dif: u32, spc: u32, fill: i32) -> i32 {
  unsafe { DrawCapsule3D(p0, p1, r, div_num, dif, spc, fill) }
}

/// (move)
pub fn draw_cone_3d(top: VECTOR, bottom: VECTOR, r: f32, div_num: i32,
  dif: u32, spc: u32, fill: i32) -> i32 {
  unsafe { DrawCone3D(top, bottom, r, div_num, dif, spc, fill) }
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
