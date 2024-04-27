//! ext dx bridge for DxLib
//!

use crate::dx::*;

pub mod music;
pub mod sound;
pub mod graph;
pub mod shader;
pub mod light;
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

/// calc norm or auto calc normalize(norm) later by HLSL
/// nf: true: normalize, false: skip normalize
pub fn calc_norm(vs: &mut Vec<VERTEX3DSHADER>, nf: bool) {
  let npolys = vs.len() / 3;
  for f in 0..npolys {
    let t = (0..3).into_iter().map(|k| &vs[f * 3 + k].pos).collect::<Vec<_>>();
    let a = VECTOR::new(t[2].x - t[1].x, t[2].y - t[1].y, t[2].z - t[1].z);
    let b = VECTOR::new(t[1].x - t[0].x, t[1].y - t[0].y, t[1].z - t[0].z);
    let v = [
      a.y * b.z - a.z * b.y,
      a.z * b.x - a.x * b.z,
      a.x * b.y - a.y * b.x];
    let n = match nf {
    true => { // normalize
      let mut d = v.iter().map(|&p| p * p).sum::<f32>().sqrt();
      if d < 0.000001 { d = 1.0 };
      let e = v.iter().map(|&p| p / d).collect::<Vec<_>>().try_into().unwrap();
      VECTOR::get(&e)
    },
    false => VECTOR::get(&v) // auto calc normalize(norm) later by HLSL
    };
    for k in 0..3 { vs[f * 3 + k].norm = n.clone(); }
  }
}

/// set -Y to convert culling CCW(GL) to CW(DX) (front &lt;-&gt; back)
/// - vts_gl 0 1 2 3 (1 2 0 2 3 0) to vss (0 3 2 0 2 1)
/// - tex: true: texture color, false: vertex color
/// - result vs: Vec&lt;VERTEX3DSHADER&gt;
pub fn from_vts_gl(vts: &Vec<VT>, offset: &POS, scale: f32, tex: bool) ->
  Vec<VERTEX3DSHADER> {
  let vpf = vts.len();
  let npolys = vpf - 2;
  let mut tbl: Vec<usize> = vec![0; 3 * npolys];
  for i in 0..npolys {
    tbl[i * 3] = 0;
    tbl[i * 3 + 1] = vpf - 1 - i;
    tbl[i * 3 + 2] = vpf - 2 - i;
  }
  let mut vs: Vec<VERTEX3DSHADER> = vec![];
  for &k in tbl.iter() { // vts[k] as [nfaces][0 3 2 0 2 1]
    let p = [
      offset.x + vts[k].pos.x * scale,
      offset.y - vts[k].pos.y * scale, // CCW to CW (front <-> back)
      offset.z + vts[k].pos.z * scale];
    let pos = VECTOR::get(&p); // shape <> CW + CW
    let spos = FLOAT4::zeros();
    let v = [vts[k].pos.x, vts[k].pos.y, vts[k].pos.z];
    let norm = VECTOR::get(&v); // calc later
    let tan = VECTOR::zeros();
    let binorm = VECTOR::zeros();
    // (all white and alpha max when use texture)
    let dif = if tex { COLOR_U8::new(255, 255, 255, 255) } // diffuse
      else { COLOR_U8::from_float4(&vts[k].pos) };
    let spc = if tex { COLOR_U8::get(&[0, 0, 255, 255]) } // specular
      else { COLOR_U8::new(255, 255, 255, 255) };
    let uv = vts[k].uv.clone(); // texture UV
    let suv = FLOAT2::zeros();
    vs.push(VERTEX3DSHADER{pos, spos, norm, tan, binorm, dif, spc, uv, suv});
  }
  calc_norm(&mut vs, false);
  vs
}

/// set -Y to convert culling CCW(GL) to CW(DX) (front &lt;-&gt; back)
/// - vts_gl 0 1 2 3 (1 2 0 2 3 0) to vss (0 3 2 0 2 1)
/// - tex: true: texture color, false: vertex color
/// - result vss: Vec&lt;Vec&lt;VERTEX3DSHADER&gt;&gt;
pub fn from_vec_vts_gl(vec_vts: &Vec<Vec<VT>>,
  offset: &POS, scale: f32, tex: bool) -> Vec<Vec<VERTEX3DSHADER>> {
  vec_vts.iter().map(|v| from_vts_gl(v, offset, scale, tex)).collect()
}

/// set -Y to convert culling CCW(GL) to CW(DX) (front &lt;-&gt; back)
/// - vts_gl 0 1 2 3 (1 2 0 2 3 0) to vss (0 3 2 0 2 1)
/// - tex: true: texture color, false: vertex color
/// - result vsss: Vec&lt;Vec&lt;Vec&lt;VERTEX3DSHADER&gt;&gt;&gt;
pub fn from_vec_vec_vts_gl(vec_vec_vts: &Vec<Vec<Vec<VT>>>,
  offset: &POS, scale: f32, tex: bool) -> Vec<Vec<Vec<VERTEX3DSHADER>>> {
  vec_vec_vts.iter().map(|f| f.iter().map(|v|
    from_vts_gl(v, offset, scale, tex)).collect()).collect()
}
