//! ext dx bridge for DxLib
//!

use std::error::Error;
use std::collections::HashMap;

use crate::dx::*;

pub trait Tr {
  fn handle(&self) -> i32;
  fn dispose(&mut self);
  fn volume(&self, _v: i32) {} // default do nothing
  fn stop(&self) {} // default do nothing
  fn play(&self, _t: i32, _f: i32) {} // default do nothing
}

pub struct Sound {
  pub h: i32
}

impl Tr for Sound {
  fn handle(&self) -> i32 { self.h }
  fn dispose(&mut self) {
    if self.h != 0 {
      unsafe { DeleteSoundMem(self.h, FALSE); }
      self.h = 0;
    }
  }
  fn volume(&self, v: i32) { unsafe { ChangeVolumeSoundMem(v, self.h); } }
  fn stop(&self) { unsafe { StopSoundMem(self.h); } }
  fn play(&self, t: i32, f: i32) { unsafe { PlaySoundMem(self.h, t, f); } }
}

impl Drop for Sound {
  fn drop(&mut self) { self.dispose(); }
}

impl Sound {
  pub fn load_mem(n: &String) -> Self {
    Sound{h: unsafe { LoadSoundMem(n.as_ptr()) } }
  }
}

pub struct Tdx {
  pub tbl: HashMap<i32, Box<dyn Tr>>
}

impl Tdx {
  pub fn new() -> Result<Self, Box<dyn Error>> {
    if unsafe { DxLib_Init() } == -1 { return Err("Cannot init DxLib".into()) }
    Ok(Tdx{tbl: HashMap::new()})
  }

  pub fn load_sound_mem(&mut self, n: &String) -> &Box<dyn Tr> { // &Box<Sound>
    let o = Sound::load_mem(n);
    let h = o.handle();
    self.tbl.insert(h, Box::new(o));
    self.tbl.get(&h).expect("get")
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

pub type UV = FLOAT2;
pub type POS = FLOAT4;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VT {
  pub pos: POS,
  pub uv: UV
}

impl VT {
  pub fn new(pos: POS, uv: UV) -> Self { VT{pos, uv} }
  pub fn zeros() -> Self { VT::new(POS::zeros(), FLOAT2::zeros()) }
  pub fn get(f4: &[f32; 4], f2: &[f32; 2]) -> Self {
    VT::new(POS::get(f4), UV::get(f2))
  }
}

// set -Y to convert culling CCW(GL) to CW(DX) (front <-> back)
// vts_gl 0 1 2 3 (0 1 2 2 3 0) to vss (0 3 2 2 1 0) or (3 2 0 1 0 2)
pub fn vss_from_vts_gl(vts: &Vec<VT>, nfaces: usize, vpf: usize,
  offset: &POS, scale: f32) -> Vec<Vec<VERTEX3DSHADER>> {
  let mut vss: Vec<Vec<VERTEX3DSHADER>> = vec![];
  let tbl: [usize; 4] = [0, 3, 2, 1];
  for i in 0..nfaces {
    let mut vs: Vec<VERTEX3DSHADER> = vec![];
    for j in 0..vpf {
      let k = i * vpf + tbl[j]; // j[0 1 2 3] to vts[k] as [nfaces][0 3 2 1]
      let p = [
        offset.x + vts[k].pos.x * scale,
        offset.y - vts[k].pos.y * scale, // CCW to CW (front <-> back)
        offset.z + vts[k].pos.z * scale];
      let pos = VECTOR::get(&p); // shape <> CW + CW
      let spos = FLOAT4::zeros();
      let norm = VECTOR::zeros();
      let tan = VECTOR::zeros();
      let binorm = VECTOR::zeros();
      // (all white and alpha max when use texture)
      let dif = COLOR_U8::new(255, 255, 255, 255); // diffuse
      let spc = COLOR_U8::zeros();
      let uv = vts[k].uv.clone(); // texture UV
      let suv = FLOAT2::zeros();
      vs.push(VERTEX3DSHADER{pos, spos, norm, tan, binorm, dif, spc, uv, suv});
    }
    // add 2 vertices to make shape <> as CW + CW {0 3 2 1} (0 3 2 2 1 0)
    vs.push(vs[3].clone()); // vs[4]
    vs.push(vs[0].clone()); // vs[5]
    vs[3] = vs[2].clone(); // vs[3]
    vss.push(vs);
  }
  vss
}
