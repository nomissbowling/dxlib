//! music ext dx bridge for DxLib
//!

use crate::{dx::*, ext::tdx::*};

/// Music
pub struct Music {
  /// to be disposed
  pub d: bool,
  /// handle
  pub h: i32
}

/// Tr for Music
impl Tr for Music {
  /// as music
  fn as_music(&self) -> Music { Music{d: false, h: self.h} }

  /// handle
  fn handle(&self) -> i32 { self.h }
  /// dispose
  fn dispose(&mut self) {
    if self.d && self.h != 0 {
      unsafe { DeleteMusicMem(self.h); }
      self.h = 0;
    }
  }
}

/// Drop for Music
impl Drop for Music {
  /// drop
  fn drop(&mut self) { self.dispose(); }
}

/// Music
impl Music {
  /// load mem
  pub fn load_mem(n: &String) -> Self {
    Music{d: true, h: unsafe { LoadMusicMem(n.as_ptr()) } }
  }
  /// volume
  pub fn volume(&self, v: i32) { unsafe { SetVolumeMusicMem(v, self.h); } }
  /// stop
  pub fn stop(&self) { unsafe { StopMusicMem(self.h); } }
  /// play
  pub fn play(&self, t: i32) { unsafe { PlayMusicMem(self.h, t); } }
}
