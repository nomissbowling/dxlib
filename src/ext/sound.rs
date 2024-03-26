//! sound ext dx bridge for DxLib
//!

use crate::{dx::*, ext::tdx::*};

/// Sound
pub struct Sound {
  /// to be disposed
  pub d: bool,
  /// handle
  pub h: i32
}

/// Tr for Sound
impl Tr for Sound {
  /// as sound
  fn as_sound(&self) -> Option<Sound> { Some(Sound{d: false, h: self.h}) }

  /// handle
  fn handle(&self) -> i32 { self.h }
  /// dispose
  fn dispose(&mut self) {
    if self.d && self.h != 0 {
      unsafe { DeleteSoundMem(self.h, FALSE); }
      self.h = 0;
    }
  }
}

/// Drop for Sound
impl Drop for Sound {
  /// drop
  fn drop(&mut self) { self.dispose(); }
}

/// Sound
impl Sound {
  /// load mem
  pub fn load_mem(n: &String) -> Self {
    Sound{d: true, h: unsafe { LoadSoundMem(n.as_ptr()) } }
  }
  /// volume
  pub fn volume(&self, v: i32) { unsafe { ChangeVolumeSoundMem(v, self.h); } }
  /// stop
  pub fn stop(&self) { unsafe { StopSoundMem(self.h); } }
  /// play
  pub fn play(&self, t: i32, f: i32) { unsafe { PlaySoundMem(self.h, t, f); } }
}
