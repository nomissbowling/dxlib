//! shader ext dx bridge for DxLib
//!

use crate::{dx::*, ext::tdx::*};

/// VertexShader
pub struct VertexShader {
  /// to be disposed
  pub d: bool,
  /// handle
  pub h: i32
}

/// Tr for VertexShader
impl Tr for VertexShader {
  /// as vertex shader
  fn as_vertex_shader(&self) -> Option<VertexShader> {
    Some(VertexShader{d: false, h: self.h})
  }

  /// handle
  fn handle(&self) -> i32 { self.h }
  /// dispose
  fn dispose(&mut self) {
    if self.d && self.h != 0 {
      unsafe { DeleteShader(self.h); }
      self.h = 0;
    }
  }
}

/// Drop for VertexShader
impl Drop for VertexShader {
  /// drop
  fn drop(&mut self) { self.dispose(); }
}

/// VertexShader
impl VertexShader {
  /// load
  pub fn load(n: &String) -> Self {
    VertexShader{d: true, h: unsafe { LoadVertexShader(n.as_ptr()) } }
  }
  /// set shader
  pub fn set_shader(&self) {
    unsafe { SetUseVertexShader(self.h); }
  }
}

/// PixelShader
pub struct PixelShader {
  /// to be disposed
  pub d: bool,
  /// handle
  pub h: i32
}

/// Tr for PixelShader
impl Tr for PixelShader {
  /// as pixel shader
  fn as_pixel_shader(&self) -> Option<PixelShader> {
    Some(PixelShader{d: false, h: self.h})
  }

  /// handle
  fn handle(&self) -> i32 { self.h }
  /// dispose
  fn dispose(&mut self) {
    if self.d && self.h != 0 {
      unsafe { DeleteShader(self.h); }
      self.h = 0;
    }
  }
}

/// Drop for PixelShader
impl Drop for PixelShader {
  /// drop
  fn drop(&mut self) { self.dispose(); }
}

/// PixelShader
impl PixelShader {
  /// load
  pub fn load(n: &String) -> Self {
    PixelShader{d: true, h: unsafe { LoadPixelShader(n.as_ptr()) } }
  }
  /// set shader
  pub fn set_shader(&self) {
    unsafe { SetUsePixelShader(self.h); }
  }
}

/// GeometryShader
pub struct GeometryShader {
  /// to be disposed
  pub d: bool,
  /// handle
  pub h: i32
}

/// Tr for GeometryShader
impl Tr for GeometryShader {
  /// as geometry shader
  fn as_geometry_shader(&self) -> Option<GeometryShader> {
    Some(GeometryShader{d: false, h: self.h})
  }

  /// handle
  fn handle(&self) -> i32 { self.h }
  /// dispose
  fn dispose(&mut self) {
    if self.d && self.h != 0 {
      unsafe { DeleteShader(self.h); }
      self.h = 0;
    }
  }
}

/// Drop for GeometryShader
impl Drop for GeometryShader {
  /// drop
  fn drop(&mut self) { self.dispose(); }
}

/// GeometryShader
impl GeometryShader {
  /// load
  pub fn load(n: &String) -> Self {
    GeometryShader{d: true, h: unsafe { LoadGeometryShader(n.as_ptr()) } }
  }
  /// set shader
  pub fn set_shader(&self) {
    unsafe { SetUseGeometryShader(self.h); }
  }
}
