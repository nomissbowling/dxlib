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

/// set -Y to convert culling CCW(GL) to CW(DX) (front &lt;-&gt; back)
/// - vts_gl 0 1 2 3 (1 2 0 2 3 0) to vss (0 3 2 0 2 1)
/// - tex: true: texture color, false: vertex color
/// - result vss: Vec&lt;Vec&lt;VERTEX3DSHADER&gt;&gt;
pub fn vss_from_vts_gl(vts: &Vec<VT>, nfaces: usize, vpf: usize,
  offset: &POS, scale: f32, tex: bool) -> Vec<Vec<VERTEX3DSHADER>> {
  let mut vss: Vec<Vec<VERTEX3DSHADER>> = vec![];
  let npolys = vpf - 2;
  let mut tbl: Vec<usize> = vec![0; 3 * npolys];
  for i in 0..npolys {
    tbl[i * 3] = 0;
    tbl[i * 3 + 1] = vpf - 1 - i;
    tbl[i * 3 + 2] = vpf - 2 - i;
  }
  for i in 0..nfaces {
    let mut vs: Vec<VERTEX3DSHADER> = vec![];
    for &j in tbl.iter() {
      let k = i * vpf + j; // vts[k] as [nfaces][0 3 2 0 2 1]
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
    vss.push(vs);
  }
  vss
}
