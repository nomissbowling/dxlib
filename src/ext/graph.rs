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
  pub fn load(n: &String) -> Self {
    Graph{h: unsafe { LoadGraph(n.as_ptr()) } }
  }
}
