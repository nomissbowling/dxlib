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
}

impl Drop for Graph {
  fn drop(&mut self) { self.dispose(); }
}

impl Graph {
  pub fn load(n: &String) -> Self {
    Graph{h: unsafe { LoadGraph(n.as_ptr()) } }
  }
}
