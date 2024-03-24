//! shader ext dx bridge for DxLib
//!

use crate::{dx::*, ext::*};

pub struct VertexShader {
  pub h: i32
}

impl Tr for VertexShader {
  fn handle(&self) -> i32 { self.h }
  fn dispose(&mut self) {
    if self.h != 0 {
      unsafe { DeleteShader(self.h); }
      self.h = 0;
    }
  }
  fn set_shader(&self) {
    unsafe { SetUseVertexShader(self.h); }
  }
}

impl Drop for VertexShader {
  fn drop(&mut self) { self.dispose(); }
}

impl VertexShader {
  pub fn load(n: &String) -> Self {
    VertexShader{h: unsafe { LoadVertexShader(n.as_ptr()) } }
  }
}

pub struct PixelShader {
  pub h: i32
}

impl Tr for PixelShader {
  fn handle(&self) -> i32 { self.h }
  fn dispose(&mut self) {
    if self.h != 0 {
      unsafe { DeleteShader(self.h); }
      self.h = 0;
    }
  }
  fn set_shader(&self) {
    unsafe { SetUsePixelShader(self.h); }
  }
}

impl Drop for PixelShader {
  fn drop(&mut self) { self.dispose(); }
}

impl PixelShader {
  pub fn load(n: &String) -> Self {
    PixelShader{h: unsafe { LoadPixelShader(n.as_ptr()) } }
  }
}

pub struct GeometryShader {
  pub h: i32
}

impl Tr for GeometryShader {
  fn handle(&self) -> i32 { self.h }
  fn dispose(&mut self) {
    if self.h != 0 {
      unsafe { DeleteShader(self.h); }
      self.h = 0;
    }
  }
  fn set_shader(&self) {
    unsafe { SetUseGeometryShader(self.h); }
  }
}

impl Drop for GeometryShader {
  fn drop(&mut self) { self.dispose(); }
}

impl GeometryShader {
  pub fn load(n: &String) -> Self {
    GeometryShader{h: unsafe { LoadGeometryShader(n.as_ptr()) } }
  }
}
