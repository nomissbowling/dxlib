//! shader ext dx bridge for DxLib
//!

use crate::{dx::*, ext::tdx::*};

/// ConstantBuffer for DX11
pub struct ConstantBuffer {
  /// to be disposed
  pub d: bool,
  /// handle
  pub h: i32,
  /// n: number of FLOAT4 (alloc n * 4 * sizeof f32)
  pub n: i32,
  /// s: slot on the shader
  pub s: i32
}

/// Tr for ConstantBuffer
impl Tr for ConstantBuffer {
  /// as constant buffer
  fn as_constant_buffer(&self) -> ConstantBuffer {
    ConstantBuffer{d: false, h: self.h, n: self.n, s: self.s}
  }

  /// handle
  fn handle(&self) -> i32 { self.h }
  /// dispose
  fn dispose(&mut self) {
    if self.d && self.h != 0{
      unsafe { DeleteShaderConstantBuffer(self.h); }
      self.h = 0;
    }
  }
}

/// Drop for ConstantBuffer
impl Drop for ConstantBuffer {
  /// drop
  fn drop(&mut self) { self.dispose(); }
}

/// ConstantBuffer for DX11
impl ConstantBuffer {
  /// create
  /// - n: number of FLOAT4 (alloc n * 4 * sizeof f32)
  /// - s: slot
  pub fn create(n: i32, s: i32) -> Self {
    let sz = n * std::mem::size_of::<FLOAT4>() as i32;
    ConstantBuffer{d: true, h: unsafe { CreateShaderConstantBuffer(sz) }, n, s}
  }
  /// as slice mut
  pub fn as_slice_mut(&self) -> &mut [FLOAT4] {
    unsafe { std::slice::from_raw_parts_mut(self.ptr_mut(), self.n as usize) }
  }
  /// prt mut
  pub fn ptr_mut(&self) -> *mut FLOAT4 {
    unsafe { GetBufferShaderConstantBuffer(self.h) }
  }
  /// update
  pub fn update(&self) -> i32 {
    unsafe { UpdateShaderConstantBuffer(self.h) }
  }
  /// set to slot (use shader.set_const(&cb) instead of this inner function)
  /// - ts: DX_SHADERTYPE_VERTEX DX_SHADERTYPE_PIXEL etc
  /// - slot: any slot or cb.s
  pub fn set_to_slot(&self, ts: i32, slot: i32) -> i32 {
    unsafe { SetShaderConstantBuffer(self.h, ts, slot) }
  }
}

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
  fn as_vertex_shader(&self) -> VertexShader {
    VertexShader{d: false, h: self.h}
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

/// Ts for VertexShader
impl Ts for VertexShader {
  /// for DX11
  fn set_const(&self, cb: &ConstantBuffer) -> i32 {
    cb.set_to_slot(DX_SHADERTYPE_VERTEX, cb.s)
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
  fn as_pixel_shader(&self) -> PixelShader {
    PixelShader{d: false, h: self.h}
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

/// Ts for PixelShader
impl Ts for PixelShader {
  /// for DX11
  fn set_const(&self, cb: &ConstantBuffer) -> i32 {
    cb.set_to_slot(DX_SHADERTYPE_PIXEL, cb.s)
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
  fn as_geometry_shader(&self) -> GeometryShader {
    GeometryShader{d: false, h: self.h}
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

/// Ts for GeometryShader
impl Ts for GeometryShader {
  /// for DX11
  fn set_const(&self, cb: &ConstantBuffer) -> i32 {
    cb.set_to_slot(DX_SHADERTYPE_GEOMETRY, cb.s)
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
