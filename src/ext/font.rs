//! font ext dx bridge for DxLib
//!

use crate::{dx::*, ext::tdx::*};

/// Font
pub struct Font {
  /// to be disposed
  pub d: bool,
  /// handle
  pub h: i32
}

/// Tr for Font
impl Tr for Font {
  /// as font
  fn as_font(&self) -> Option<Font> { Some(Font{d: false, h: self.h}) }

  /// handle
  fn handle(&self) -> i32 { self.h }
  /// dispose
  fn dispose(&mut self) {
    if self.d && self.h != 0 {
      unsafe { DeleteFontToHandle(self.h); }
      self.h = 0;
    }
  }
}

/// Drop for Font
impl Drop for Font {
  /// drop
  fn drop(&mut self) { self.dispose(); }
}

/// Font
impl Font {
  /// only system fonts (handle = -1)
  pub fn create(n: &str, sz: i32, thick: i32,
    fonttype: i32, charset: i32, edgesz: i32, italic: i32) -> Self {
    Font{d: true, h: unsafe { CreateFontToHandle(n.as_ptr(), sz, thick,
      fonttype, charset, edgesz, italic, -1) } }
  }
  /// load fontdata.dft (fixed size italic etc) (edgesz = 0)
  pub fn load_data(n: &String) -> Self {
    Font{d: true, h: unsafe { LoadFontDataToHandle(n.as_ptr(), 0) } }
  }
  /// draw string
  pub fn draw_string(&self, x: i32, y: i32, s: &String, c: u32, e: u32, v: i32) {
    unsafe { DrawStringToHandle(x, y, s.as_ptr(), c, self.h, e, v); }
  }
  /// draw bytes
  pub fn draw_bytes(&self, x: i32, y: i32, b: &[u8], c: u32, e: u32, v: i32) {
    unsafe { DrawStringToHandle(x, y, b.as_ptr(), c, self.h, e, v); }
  }
}
