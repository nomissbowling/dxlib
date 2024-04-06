//! demo dx bridge for DxLib
//!

use crate::{dx::*, ext::*};

use fullerene::c60::{C60, C60Center};
use fullerene::dodecahedron::{Dodecahedron, DodecahedronCenter};
use fullerene::icosahedron::Icosahedron;
use fullerene::TUV;
use num::Float;

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

/// cube square x 6 faces on the one texture (gl order)
pub fn gen_c6f_gl() -> Vec<Vec<VT>> {
  let uvs = [
    [[0.25, 0.50], [0.50, 0.50], [0.50, 0.25], [0.25, 0.25]], // 5137
    [[0.25, 0.75], [0.25, 1.00], [0.50, 1.00], [0.50, 0.75]], // 4620
    [[0.50, 0.25], [0.50, 0.00], [0.25, 0.00], [0.25, 0.25]], // 3267
    [[0.50, 0.50], [0.25, 0.50], [0.25, 0.75], [0.50, 0.75]], // 1540
    [[0.00, 0.25], [0.00, 0.50], [0.25, 0.50], [0.25, 0.25]], // 6457
    [[0.75, 0.75], [0.75, 0.50], [0.50, 0.50], [0.50, 0.75]]]; // 2310
  let mut vec_vts = gen_vec_vts_gl();
  for (j, f) in uvs.iter().enumerate() {
    for (i, uv) in f.iter().enumerate() {
      vec_vts[j][i].uv = UV::get(uv);
    }
  }
  vec_vts
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

/// f_to_f32
pub fn f_to_f32<F: Float>(v: &[F]) -> Vec<f32> {
  v.iter().map(|i| i.to_f32().unwrap()).collect()
}

/// f_to_f64
pub fn f_to_f64<F: Float>(v: &[F]) -> Vec<f64> {
  v.iter().map(|i| i.to_f64().unwrap()).collect()
}

/// vtx to vt
pub fn gen_vt<F: Float>(ph: &impl TUV<F>, i: usize, tf: bool,
  fi: usize, n: usize, vi: usize, ii: usize, c: bool) -> VT {
  let p = f_to_f32(&ph.ref_vtx()[i]);
  let uv = f_to_f32(&match tf {
  true => ph.get_uv_t(fi, vi, ii), // on the one texture
  false => ph.get_uv_f(n, vi, ii, c)}); // texture each face
  VT::get(&[p[0], p[1], p[2], 1.0], &[uv[0], uv[1]])
}

/// polyhedron faces by Vec N of Vec P(polygon) indexed triangles (gl order)
pub fn phf<F: Float>(ph: &impl TUV<F>, tf: bool, c: bool) -> Vec<Vec<Vec<VT>>> {
  ph.ref_tri().iter().enumerate().map(|(fi, f)|
    f.iter().enumerate().map(|(vi, v)|
      v.iter().enumerate().map(|(ii, &i)| {
        gen_vt(ph, i as usize, tf, fi, f.len(), vi, ii, c)
      }).collect()
    ).collect()
  ).collect()
}

/// icosahedron (gl order)
pub fn gen_icosahedron(tf: bool) -> Vec<Vec<Vec<VT>>> {
  phf(&Icosahedron::new(1.0f32), tf, false)
}

/// dodecahedron (gl order)
pub fn gen_dodecahedron(tf: bool) -> Vec<Vec<Vec<VT>>> {
  phf(&Dodecahedron::new(1.0f32), tf, false)
}

/// dodecahedron center (gl order)
pub fn gen_dodecahedron_center(tf: bool) -> Vec<Vec<Vec<VT>>> {
  phf(&DodecahedronCenter::new(1.0f32), tf, true)
}

/// c60 (gl order)
pub fn gen_c60(tf: bool) -> Vec<Vec<Vec<VT>>> {
  phf(&C60::new(1.0f32), tf, false)
}

/// c60 center (gl order)
pub fn gen_c60_center(tf: bool) -> Vec<Vec<Vec<VT>>> {
  phf(&C60Center::new(1.0f32), tf, true)
}
