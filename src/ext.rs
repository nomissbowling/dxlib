//! ext dx bridge for DxLib
//!

use crate::dx::*;

pub mod music;
pub mod sound;
pub mod graph;
pub mod shader;
pub mod font;
pub mod tdx;

/// UV
pub type UV = FLOAT2;
/// POS
pub type POS = FLOAT4;

/// VT
#[derive(Debug, Clone)]
#[repr(C)]
pub struct VT {
  /// pos
  pub pos: POS,
  /// uv
  pub uv: UV
}

/// VT
impl VT {
  /// constructor
  pub fn new(pos: POS, uv: UV) -> Self { VT{pos, uv} }
  /// zeros
  pub fn zeros() -> Self { VT::new(POS::zeros(), FLOAT2::zeros()) }
  /// get
  pub fn get(f4: &[f32; 4], f2: &[f32; 2]) -> Self {
    VT::new(POS::get(f4), UV::get(f2))
  }
}

/// set -Y to convert culling CCW(GL) to CW(DX) (front <-> back)
/// - vts_gl 0 1 2 3 (0 1 2 2 3 0) to vss (0 3 2 2 1 0) or (3 2 0 1 0 2)
/// - tex: true: texture color, false: vertex color
pub fn vss_from_vts_gl(vts: &Vec<VT>, nfaces: usize, vpf: usize,
  offset: &POS, scale: f32, tex: bool) -> Vec<Vec<VERTEX3DSHADER>> {
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
      let dif = if tex { COLOR_U8::new(255, 255, 255, 255) } // diffuse
        else { COLOR_U8::from_float4(&vts[k].pos) };
      let spc = COLOR_U8::zeros(); // specular
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
