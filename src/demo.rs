//! demo dx bridge for DxLib
//!

use crate::{dx::*, ext::*};

pub mod dum;
pub mod typ;

/// square bended shape Z (not gl order)
/// - result vert: Vec&lt;VERTEX3D&gt;
pub fn gen_vert() -> Vec<VERTEX3DSHADER> {
  let mut vert: Vec<VERTEX3DSHADER> = [ // (shape, diffuse, texture)
    ([-128.0, -256.0, 128.0], [255, 255, 255, 255], [0.0, 0.0]),
    ([128.0, -384.0, 128.0], [128, 128, 128, 255], [1.0, 0.0]),
    ([-128.0, -384.0, -128.0], [255, 128, 255, 255], [0.0, 1.0]),
    ([128.0, -256.0, -128.0], [255, 255, 255, 255], [1.0, 1.0])
  ].iter().map(|t| {
    let pos = VECTOR::get(&t.0); // shape Z CW[0 1 2] + CCW[1 2 3]
    let spos = FLOAT4::zeros();
    let norm = VECTOR::zeros();
    let tan = VECTOR::zeros();
    let binorm = VECTOR::zeros();
    let dif = COLOR_U8::get(&t.1); // diffuse (blend vetex color with texture)
    let spc = COLOR_U8::zeros(); // specular
    let uv = FLOAT2::get(&t.2); // texture UV shape Z CW + CCW
    let suv = FLOAT2::zeros();
    VERTEX3DSHADER{pos, spos, norm, tan, binorm, dif, spc, uv, suv}
  }).collect();
  // add 2 vertices to make shape Z as CW[0 1 2] + CW[3 2 1]
  vert.push(vert[2].clone()); // vert[4]
  vert.push(vert[1].clone()); // vert[5]
  vert
}

/// square bended shape &lt;&gt; (gl order)
pub fn gen_vert_gl() -> Vec<VT> {
  let vert_gl: Vec<VT> = [ // (for GL)
    ([-1.0, -1.5, 0.0, 1.0], [0.0, 1.0]),
    ([-1.0, -2.0, 1.0, 1.0], [1.0, 1.0]),
    ([1.0, -2.0, 0.0, 1.0], [1.0, 0.0]),
    ([1.0, -1.5, -0.5, 1.0], [0.0, 0.0])
  ].iter().map(|t| VT::get(&t.0, &t.1)).collect();
  vert_gl
}

/// polygon (gl order)
pub fn gen_poly_gl(n: usize) -> Vec<VT> {
  let poly_gl: Vec<VT> = (0..n).into_iter().map(|i| {
    let t = 2.0 * std::f32::consts::PI * i as f32 / n as f32;
    let p = [t.cos(), t.sin() - 1.0, t.sin(), 1.0];
    let uv = [(1.0 + t.cos()) / 2.0, 1.0 - (1.0 + t.sin()) / 2.0];
    VT::get(&p, &uv)
  }).collect();
  poly_gl
}

/// square x 6 (gl order)
pub fn gen_vec_vts_gl() -> Vec<Vec<VT>> {
  let vts_gl = [ // (for GL) FrontFace::Ccw culling Face::Back
    // +X (1, 0, 0) right
    ([1.0, -1.0, 1.0, 1.0], [0.0, 1.0]),
    ([1.0, -1.0, -1.0, 1.0], [1.0, 1.0]),
    ([1.0, 1.0, -1.0, 1.0], [1.0, 0.0]),
    ([1.0, 1.0, 1.0, 1.0], [0.0, 0.0]),
    // -X (-1, 0, 0) left
    ([-1.0, -1.0, 1.0, 1.0], [0.0, 1.0]),
    ([-1.0, 1.0, 1.0, 1.0], [1.0, 1.0]),
    ([-1.0, 1.0, -1.0, 1.0], [1.0, 0.0]),
    ([-1.0, -1.0, -1.0, 1.0], [0.0, 0.0]),
    // +Y (0, 1, 0) back
    ([1.0, 1.0, -1.0, 1.0], [0.0, 1.0]),
    ([-1.0, 1.0, -1.0, 1.0], [1.0, 1.0]),
    ([-1.0, 1.0, 1.0, 1.0], [1.0, 0.0]),
    ([1.0, 1.0, 1.0, 1.0], [0.0, 0.0]),
    // -Y (0, -1, 0) front
    ([1.0, -1.0, -1.0, 1.0], [0.0, 1.0]),
    ([1.0, -1.0, 1.0, 1.0], [1.0, 1.0]),
    ([-1.0, -1.0, 1.0, 1.0], [1.0, 0.0]),
    ([-1.0, -1.0, -1.0, 1.0], [0.0, 0.0]),
    // +Z (0, 0, 1) top
    ([-1.0, 1.0, 1.0, 1.0], [0.0, 1.0]),
    ([-1.0, -1.0, 1.0, 1.0], [1.0, 1.0]),
    ([1.0, -1.0, 1.0, 1.0], [1.0, 0.0]),
    ([1.0, 1.0, 1.0, 1.0], [0.0, 0.0]),
    // -Z (0, 0, -1) bottom
    ([-1.0, 1.0, -1.0, 1.0], [0.0, 1.0]),
    ([1.0, 1.0, -1.0, 1.0], [1.0, 1.0]),
    ([1.0, -1.0, -1.0, 1.0], [1.0, 0.0]),
    ([-1.0, -1.0, -1.0, 1.0], [0.0, 0.0])
  ];
  (0..6).into_iter().map(|f|
    (0..4).into_iter().map(|i| {
      let t = vts_gl[f * 4 + i];
      VT::get(&t.0, &t.1)
    }).collect::<Vec<_>>()).collect::<Vec<_>>()
}

/// any face (gl order)
pub fn gen_any_face() -> Vec<Vec<VT>> {
  [
    vec![
      ([-1.0, -1.0, 2.0, 1.0], [1.0, 1.0]),
      ([0.0, 0.0, 1.0 - 0.707, 1.0], [0.5, 1.0 - 0.866]),
      ([1.0, -1.0, 2.0, 1.0], [0.0, 1.0])],
    vec![
      ([1.0, -1.0, 2.0, 1.0], [1.0, 1.0]),
      ([0.0, 0.0, 1.0 - 0.707, 1.0], [0.5, 1.0 - 0.866]),
      ([1.0, 1.0, 2.0, 1.0], [0.0, 1.0])],
    vec![
      ([1.0, 1.0, 2.0, 1.0], [1.0, 1.0]),
      ([0.0, 0.0, 1.0 - 0.707, 1.0], [0.5, 1.0 - 0.866]),
      ([-1.0, 1.0, 2.0, 1.0], [0.0, 1.0])],
    vec![
      ([-1.0, 1.0, 2.0, 1.0], [1.0, 1.0]),
      ([0.0, 0.0, 1.0 - 0.707, 1.0], [0.5, 1.0 - 0.866]),
      ([-1.0, -1.0, 2.0, 1.0], [0.0, 1.0])],
    vec![
      ([-1.0, -1.0, 2.0, 1.0], [0.0, 1.0]),
      ([1.0, -1.0, 2.0, 1.0], [1.0, 1.0]),
      ([1.0, 1.0, 2.0, 1.0], [1.0, 0.0]),
      ([-1.0, 1.0, 2.0, 1.0], [0.0, 0.0])]
  ].iter().map(|f|
    f.iter().map(|t|
      VT::get(&t.0, &t.1)).collect::<Vec<_>>()).collect::<Vec<_>>()
}
