//! ext dx bridge for DxLib
//!

use std::rc::Rc;
use std::cell::RefCell;

use crate::dx::*;

pub mod music;
pub mod sound;
pub mod graph;
pub mod shader;
pub mod font;
pub mod tdx;

pub type RcTr = Rc<RefCell<Box<dyn Tr>>>;

pub trait Tr {
  fn handle(&self) -> i32;
  fn dispose(&mut self);
  fn volume(&self, _v: i32) {} // default do nothing
  fn stop(&self) {} // default do nothing
  fn play(&self, _t: i32, _f: i32) {} // default do nothing
  fn get_draw_screen(&self, _l: i32, _t: i32, _r: i32, _b: i32,
    _use_client_flag: i32) {}
  fn draw(&self, _x: i32, _y: i32, _f: i32) {} // default do nothing
  fn draw_rota(&self, _x: i32, _y: i32, _extrate: f64, _angle: f64,
    _trans: i32, _reversex: i32, _reversey: i32) {} // default do nothing
  fn set_to_shader(&self, _i: i32) {}
  fn set_shader(&self) {}
  fn draw_string(&self, _x: i32, _y: i32,
    _s: &String, _c: u32, _e: u32, _v: i32) {}
  fn draw_bytes(&self, _x: i32, _y: i32,
    _b: &[u8], _c: u32, _e: u32, _v: i32) {}
}

impl Tr for RcTr {
  fn handle(&self) -> i32 { self.borrow().handle() }
  fn dispose(&mut self) { self.borrow_mut().dispose(); }
  fn volume(&self, v: i32) { self.borrow().volume(v); }
  fn stop(&self) { self.borrow().stop(); }
  fn play(&self, t: i32, f: i32) { self.borrow().play(t, f); }
  fn get_draw_screen(&self, l: i32, t: i32, r: i32, b: i32,
    use_client_flag: i32) {
    self.borrow().get_draw_screen(l, t, r, b, use_client_flag);
  }
  fn draw(&self, x: i32, y: i32, f: i32) { self.borrow().draw(x, y, f); }
  fn draw_rota(&self, x: i32, y: i32, extrate: f64, angle: f64,
    trans: i32, reversex: i32, reversey: i32) {
    self.borrow().draw_rota(x, y, extrate, angle, trans, reversex, reversey);
  }
  fn set_to_shader(&self, i: i32) { self.borrow().set_to_shader(i); }
  fn set_shader(&self) { self.borrow().set_shader(); }
  fn draw_string(&self, x: i32, y: i32, s: &String, c: u32, e: u32, v: i32) {
    self.borrow().draw_string(x, y, s, c, e, v);
  }
  fn draw_bytes(&self, x: i32, y: i32, b: &[u8], c: u32, e: u32, v: i32) {
    self.borrow().draw_bytes(x, y, b, c, e, v);
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
