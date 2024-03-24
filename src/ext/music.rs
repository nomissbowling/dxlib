//! music ext dx bridge for DxLib
//!

use crate::{dx::*, ext::*};

pub struct Music {
  pub h: i32
}

impl Tr for Music {
  fn handle(&self) -> i32 { self.h }
  fn dispose(&mut self) {
    if self.h != 0 {
      unsafe { DeleteMusicMem(self.h); }
      self.h = 0;
    }
  }
  fn volume(&self, v: i32) { unsafe { SetVolumeMusicMem(v, self.h); } }
  fn stop(&self) { unsafe { StopMusicMem(self.h); } }
  fn play(&self, t: i32, _f: i32) { unsafe { PlayMusicMem(self.h, t); } }
}

impl Drop for Music {
  fn drop(&mut self) { self.dispose(); }
}

impl Music {
  pub fn load_mem(n: &String) -> Self {
    Music{h: unsafe { LoadMusicMem(n.as_ptr()) } }
  }
}
