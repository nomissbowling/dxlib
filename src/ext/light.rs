//! light ext dx bridge for DxLib
//!

use crate::{dx::*, ext::tdx::*};

/// Light
pub struct Light {
  /// to be disposed
  pub d: bool,
  /// handle
  pub h: i32
}

/// Tr for Light
impl Tr for Light {
  /// as light
  fn as_light(&self) -> Light {
    Light{d: false, h: self.h}
  }

  /// handle
  fn handle(&self) -> i32 { self.h }
  /// dispose
  fn dispose(&mut self) {
    if self.d && self.h != 0 {
      unsafe { DeleteLightHandle(self.h); }
      self.h = 0;
    }
  }
}

/// Drop for Light
impl Drop for Light {
  /// drop
  fn drop(&mut self) { self.dispose(); }
}

/// Light
impl Light {
  /// (move) create dir
  pub fn create_dir(d: VECTOR) -> Self {
    Light{d: true, h: unsafe { CreateDirLightHandle(d) } }
  }
  /// (move) create spot
  pub fn create_spot(p: VECTOR, d: VECTOR, oa: f32, ia: f32,
    rng: f32, a0: f32, a1: f32, a2: f32) -> Self {
    Light{d: true,
      h: unsafe { CreateSpotLightHandle(p, d, oa, ia, rng, a0, a1, a2) }
    }
  }
  /// (move) create point
  pub fn create_point(p: VECTOR,
    rng: f32, a0: f32, a1: f32, a2: f32) -> Self {
    Light{d: true,
      h: unsafe { CreatePointLightHandle(p, rng, a0, a1, a2) }
    }
  }
  /// set type
  pub fn set_type(&self, typ: i32) -> i32 {
    unsafe { SetLightTypeHandle(self.h, typ) }
  }
  /// set enable
  pub fn set_enable(&self, flg: i32) -> i32 {
    unsafe { SetLightEnableHandle(self.h, flg) }
  }
  /// (move)
  pub fn set_dif_color(&self, c: COLOR_F) -> i32 {
    unsafe { SetLightDifColorHandle(self.h, c) }
  }
  /// (move)
  pub fn set_spc_color(&self, c: COLOR_F) -> i32 {
    unsafe { SetLightSpcColorHandle(self.h, c) }
  }
  /// (move)
  pub fn set_amb_color(&self, c: COLOR_F) -> i32 {
    unsafe { SetLightAmbColorHandle(self.h, c) }
  }
  /// (move)
  pub fn set_direction(&self, d: VECTOR) -> i32 {
    unsafe { SetLightDirectionHandle(self.h, d) }
  }
  /// (move)
  pub fn set_position(&self, p: VECTOR) -> i32 {
    unsafe { SetLightPositionHandle(self.h, p) }
  }
  ///
  pub fn set_range_atten(&self,
    rng: f32, a0: f32, a1: f32, a2: f32) -> i32 {
    unsafe { SetLightRangeAttenHandle(self.h, rng, a0, a1, a2) }
  }
  ///
  pub fn set_angle(&self, oa: f32, ia: f32) -> i32 {
    unsafe { SetLightAngleHandle(self.h, oa, ia) }
  }
  /// get type DX_LIGHTTYPE_DIRECTIONAL etc
  pub fn get_type(&self) -> i32 {
    unsafe { GetLightTypeHandle(self.h) }
  }
  /// get enable
  pub fn get_enable(&self) -> i32 {
    unsafe { GetLightEnableHandle(self.h) }
  }
  ///
  pub fn get_dif_color(&self) -> COLOR_F {
    unsafe { GetLightDifColorHandle(self.h) }
  }
  ///
  pub fn get_spc_color(&self) -> COLOR_F {
    unsafe { GetLightSpcColorHandle(self.h) }
  }
  ///
  pub fn get_amb_color(&self) -> COLOR_F {
    unsafe { GetLightAmbColorHandle(self.h) }
  }
  ///
  pub fn get_direction(&self) -> VECTOR {
    unsafe { GetLightDirectionHandle(self.h) }
  }
  ///
  pub fn get_position(&self) -> VECTOR {
    unsafe { GetLightPositionHandle(self.h) }
  }
  ///
  pub fn get_range_atten(&self,
    rng: &mut f32, a0: &mut f32, a1: &mut f32, a2: &mut f32) -> i32 {
    unsafe { GetLightRangeAttenHandle(self.h,
      rng as *mut f32, a0 as *mut f32, a1 as *mut f32, a2 as *mut f32) }
  }
  ///
  pub fn get_angle(&self, oa: &mut f32, ia: &mut f32) -> i32 {
    unsafe { GetLightAngleHandle(self.h, oa as *mut f32, ia as *mut f32) }
  }
}
