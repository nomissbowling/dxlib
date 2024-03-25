//! font ext dx bridge for DxLib
//!

use crate::{dx::*, ext::*};

pub struct Font {
  pub h: i32
}

impl Tr for Font {
  fn handle(&self) -> i32 { self.h }
  fn dispose(&mut self) {
    if self.h != 0 {
      unsafe { DeleteFontToHandle(self.h); }
      self.h = 0;
    }
  }
  fn draw_string(&self, x: i32, y: i32, s: &String, c: u32, e: u32, v: i32) {
    unsafe { DrawStringToHandle(x, y, s.as_ptr(), c, self.h, e, v); }
  }
  fn draw_bytes(&self, x: i32, y: i32, b: &[u8], c: u32, e: u32, v: i32) {
    unsafe { DrawStringToHandle(x, y, b.as_ptr(), c, self.h, e, v); }
  }
}

impl Drop for Font {
  fn drop(&mut self) { self.dispose(); }
}

impl Font {
  /// only system fonts
  pub fn create(n: &str, sz: i32, thick: i32,
    fonttype: i32, charset: i32, edgesz: i32, italic: i32) -> Self {
    Font{h: unsafe { CreateFontToHandle(n.as_ptr(), sz, thick,
      fonttype, charset, edgesz, italic, -1) } } // handle = -1
  }
  /// load fontdata.dft (fixed size italic etc)
  pub fn load_data(n: &String) -> Self {
    Font{h: unsafe { LoadFontDataToHandle(n.as_ptr(), 0) } } // edgesz = 0
  }
}
