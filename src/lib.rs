#![doc(html_root_url = "https://docs.rs/dxlib/0.5.1")]
//! dxlib dll for Rust
//!

pub mod dx;
pub mod ext;
pub mod demo;

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  // use super::*;
  use crate::dx::{COLOR_F, COLOR_U8, FLOAT4};
  use crate::demo;

  /// test COLOR_F
  #[test]
  fn test_color_f() {
    assert_eq!(COLOR_F::new(0.0, 0.0, 1.0, 1.0),
      COLOR_F::get(&[0.0, 0.0, 1.0, 1.0]));
    assert_eq!(COLOR_F::new(0.0, 0.0, 0.0, 0.0),
      COLOR_F::from_u8(&COLOR_U8::new(0, 0, 0, 0)));
    assert_eq!(COLOR_F::new(1.0, 1.0, 1.0, 1.0),
      COLOR_F::from_u8(&COLOR_U8::new(255, 255, 255, 255)));
    assert_eq!(COLOR_F::new(0.0, 0.0, 1.0, 1.0),
      COLOR_F::from_u8(&COLOR_U8::new(255, 0, 0, 255)));
    assert_eq!(COLOR_F::new(0.0, 0.0, 1.0, 1.0),
      COLOR_F::from_float4(&FLOAT4::new(-2.0, -2.0, 2.0, 1.0)));
  }

  /// test COLOR_U8
  #[test]
  fn test_color_u8() {
    assert_eq!(COLOR_U8::new(255, 0, 0, 255),
      COLOR_U8::get(&[0, 0, 255, 255]));
    assert_eq!(COLOR_U8::new(0, 0, 0, 0),
      COLOR_U8::from_f(&COLOR_F::new(0.0, 0.0, 0.0, 0.0)));
    assert_eq!(COLOR_U8::new(255, 255, 255, 255),
      COLOR_U8::from_f(&COLOR_F::new(1.0, 1.0, 1.0, 1.0)));
    assert_eq!(COLOR_U8::new(127, 255, 255, 255),
      COLOR_U8::from_f(&COLOR_F::new(1.0, 1.0, 0.5, 1.0)));
    assert_eq!(COLOR_U8::new(255, 127, 255, 255),
      COLOR_U8::from_f(&COLOR_F::new(1.0, 0.5, 1.0, 1.0)));
    assert_eq!(COLOR_U8::new(255, 255, 127, 255),
      COLOR_U8::from_f(&COLOR_F::new(0.5, 1.0, 1.0, 1.0)));
    assert_eq!(COLOR_U8::new(255, 0, 0, 255),
      COLOR_U8::from_float4(&FLOAT4::new(-2.0, -2.0, 2.0, 1.0)));
    assert_eq!(COLOR_U8::new(63, 127, 255, 204),
      COLOR_U8::from_u32(0xccff7f3fu32));
    assert_eq!(COLOR_U8::new(64, 128, 255, 0).as_u32(), 0x00ff8040u32);
  }

  /// test screen
  #[test]
  fn test_screen() {
    // either typ or dum at once
    assert_eq!(demo::typ::screen("./resource/").expect("init"), ());
//    assert_eq!(demo::dum::screen("./resource/"), ());
  }
}
