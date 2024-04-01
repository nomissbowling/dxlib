//! demo dx bridge for DxLib
//!

use crate::{dx::*, ext::*};

pub mod dum;
pub mod typ;

pub const TPF_Q: i32 = 2; // triangles per quad
pub const VPF_Q: usize = 3 * TPF_Q as usize; // vertices per face quad
pub const VPF_VTS: usize = 4; // vertices per face of vts
pub const NFACES_CUBE: usize = 6; // faces of cube

/// square bended shape Z (not gl order)
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

/// square x 6 (gl order)
pub fn gen_vts_gl() -> Vec<VT> {
  let vts_gl: Vec<VT> = [ // (for GL) FrontFace::Ccw culling Face::Back
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
  ].iter().map(|t| VT::get(&t.0, &t.1)).collect();
  vts_gl
}
