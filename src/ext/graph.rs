//! graph ext dx bridge for DxLib
//!

use crate::{dx::*, ext::*};

pub struct Graph {
  pub h: i32
}

impl Tr for Graph {
  fn handle(&self) -> i32 { self.h }
  fn dispose(&mut self) {
    if self.h != 0 {
      unsafe { DeleteGraph(self.h, FALSE); }
      self.h = 0;
    }
  }
  /// clipping (use SetRestoreGraphCallback to recover full screen)
  /// - left, top, right + 1, bottom + 1
  /// - use_client_flag: default TRUE
  fn get_draw_screen(&self, l: i32, t: i32, r: i32, b: i32,
    use_client_flag: i32) {
    unsafe { GetDrawScreenGraph(l, t, r, b, self.h, use_client_flag); }
  }
  fn draw(&self, x: i32, y: i32, f: i32) {
    unsafe { DrawGraph(x, y, self.h, f); }
  }
  fn draw_rota(&self, x: i32, y: i32, extrate: f64, angle: f64,
    trans: i32, reversex: i32, reversey: i32) {
    unsafe {
      DrawRotaGraph(x, y, extrate, angle, self.h, trans, reversex, reversey);
    }
  }
  fn set_to_shader(&self, i: i32) {
    unsafe { SetUseTextureToShader(i, self.h); }
  }
}

impl Drop for Graph {
  fn drop(&mut self) { self.dispose(); }
}

impl Graph {
  /// empty instance (for get_draw_screen etc)
  /// - not_use_3d_flag: default FALSE
  pub fn make(xsz: i32, ysz: i32, not_use_3d_flag: i32) -> Self {
    Graph{h: unsafe { MakeGraph(xsz, ysz, not_use_3d_flag) } }
  }
  /// load from file
  pub fn load(n: &String) -> Self {
    Graph{h: unsafe { LoadGraph(n.as_ptr()) } }
  }
}
