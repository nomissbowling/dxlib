//! graph ext dx bridge for DxLib
//!

use crate::{dx::*, ext::tdx::*};

/// Graph
pub struct Graph {
  /// to be disposed
  pub d: bool,
  /// handle
  pub h: i32
}

/// Tr for Graph
impl Tr for Graph {
  /// as graph
  fn as_graph(&self) -> Option<Graph> { Some(Graph{d: false, h: self.h}) }

  /// handle
  fn handle(&self) -> i32 { self.h }
  /// dispose
  fn dispose(&mut self) {
    if self.d && self.h != 0 {
      unsafe { DeleteGraph(self.h, FALSE); }
      self.h = 0;
    }
  }
}

/// Drop for Graph
impl Drop for Graph {
  /// drop
  fn drop(&mut self) { self.dispose(); }
}

/// Graph
impl Graph {
  /// empty instance (for get_draw_screen etc)
  /// - not_use_3d_flag: default FALSE
  pub fn make(xsz: i32, ysz: i32, not_use_3d_flag: i32) -> Self {
    Graph{d: true, h: unsafe { MakeGraph(xsz, ysz, not_use_3d_flag) } }
  }
  /// load from file
  pub fn load(n: &String) -> Self {
    Graph{d: true, h: unsafe { LoadGraph(n.as_ptr()) } }
  }
  /// clipping (use SetRestoreGraphCallback to recover full screen)
  /// - left, top, right + 1, bottom + 1
  /// - use_client_flag: default TRUE
  pub fn get_draw_screen(&self, l: i32, t: i32, r: i32, b: i32,
    use_client_flag: i32) {
    unsafe { GetDrawScreenGraph(l, t, r, b, self.h, use_client_flag); }
  }
  /// draw to screen
  pub fn draw(&self, x: i32, y: i32, trans: i32) {
    unsafe { DrawGraph(x, y, self.h, trans); }
  }
  /// draw turn LR
  pub fn draw_turn(&self, x: i32, y: i32, trans: i32) {
    unsafe { DrawTurnGraph(x, y, self.h, trans); }
  }
  /// draw extend
  pub fn draw_extend(&self, l: i32, t: i32, r: i32, b: i32, trans: i32) {
    unsafe { DrawExtendGraph(l, t, r, b, self.h, trans); }
  }
  /// draw rotate
  pub fn draw_rota(&self, x: i32, y: i32, extrate: f64, angle: f64,
    trans: i32, reversex: i32, reversey: i32) {
    unsafe {
      DrawRotaGraph(x, y, extrate, angle, self.h, trans, reversex, reversey);
    }
  }
  /// draw modi
  pub fn draw_modi(&self, xlt: i32, ylt: i32, xrt: i32, yrt: i32,
    xrb: i32, yrb: i32, xlb: i32, ylb: i32, trans: i32) {
    unsafe {
      DrawModiGraph(xlt, ylt, xrt, yrt, xrb, yrb, xlb, ylb, self.h, trans);
    }
  }
  /// draw rect
  pub fn draw_rect(&self, x: i32, y: i32, srcx: i32, srcy: i32, w: i32, h: i32,
    trans: i32, reversex: i32, reversey: i32) {
    unsafe {
      DrawRectGraph(x, y, srcx, srcy, w, h, self.h, trans, reversex, reversey);
    }
  }
  /// draw rect extend
  pub fn draw_rect_extend(&self, l: i32, t: i32, r: i32, b: i32,
    srcx: i32, srcy: i32, w: i32, h: i32, trans: i32) {
    unsafe {
      DrawRectExtendGraph(l, t, r, b, srcx, srcy, w, h, self.h, trans);
    }
  }
  /// set to shader
  pub fn set_to_shader(&self, i: i32) {
    unsafe { SetUseTextureToShader(i, self.h); }
  }
}
