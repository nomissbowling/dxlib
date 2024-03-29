#![doc(html_root_url = "https://docs.rs/dxlib/0.2.1")]
//! dxlib dll for Rust
//!

pub mod dx;
pub mod ext;
pub mod demo;

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  // use super::*;
  use crate::demo;

  /// test screen
  #[test]
  fn test_screen() {
    // either typ or dum at once
    assert_eq!(demo::typ::screen("./resource/").expect("init"), ());
//    assert_eq!(demo::dum::screen("./resource/"), ());
  }
}
