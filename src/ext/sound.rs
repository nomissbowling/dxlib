//! sound ext dx bridge for DxLib
//!

use crate::{dx::*, ext::*};

pub struct Sound {
  pub h: i32
}

impl Tr for Sound {
  fn handle(&self) -> i32 { self.h }
  fn dispose(&mut self) {
    if self.h != 0 {
      unsafe { DeleteSoundMem(self.h, FALSE); }
      self.h = 0;
    }
  }
  fn volume(&self, v: i32) { unsafe { ChangeVolumeSoundMem(v, self.h); } }
  fn stop(&self) { unsafe { StopSoundMem(self.h); } }
  fn play(&self, t: i32, f: i32) { unsafe { PlaySoundMem(self.h, t, f); } }
}

impl Drop for Sound {
  fn drop(&mut self) { self.dispose(); }
}

impl Sound {
  pub fn load_mem(n: &String) -> Self {
    Sound{h: unsafe { LoadSoundMem(n.as_ptr()) } }
  }
}
