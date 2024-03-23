#![doc(html_root_url = "https://docs.rs/dxlib/0.0.2")]
//! dxlib dll for Rust
//!

use std::path::PathBuf;

pub mod dx;
use dx::*;

type UV = FLOAT2;
type POS = FLOAT4;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VT {
  pub pos: POS,
  pub uv: UV
}

impl VT {
  pub fn new(pos: POS, uv: UV) -> Self { VT{pos, uv} }
  pub fn zeros() -> Self { VT::new(POS::zeros(), FLOAT2::zeros()) }
  pub fn get(f4: &[f32; 4], f2: &[f32; 2]) -> Self {
    VT::new(POS::get(f4), UV::get(f2))
  }
}

pub fn dum_screen() {
unsafe {
  SetOutApplicationLogValidFlag(FALSE);
  ChangeWindowMode(TRUE); // not full screen
  SetGraphMode(640, 480, 32, 60); // 32 bit 60 fps
  let u8t: &[u8] = &[0x83, 0x65, 0x83, 0x58, 0x83, 0x67, 0x00]; // in cp932
  SetMainWindowText(u8t.as_ptr());
  if DxLib_Init() == -1 { return; }

  let base = PathBuf::from("./resource/");
  let mf = base.join("_decision3_.wav\0");
  InitMusicMem();
  let mh = LoadMusicMem(mf.to_str().expect("str").as_ptr());
  SelectMidiMode(DX_MIDIMODE_MCI); // DX_MIDIMODE_DM DX_MIDIMODE_MCI (default)
  SetVolumeMusicMem(96, mh);
  PlayMusicMem(mh, DX_PLAYTYPE_BACK);

  SetDrawScreen(DX_SCREEN_WORK);
  ClearDrawScreen(NULL);

  SetDrawScreen(DX_SCREEN_BACK);
  ClearDrawScreen(NULL);
  for r in 0..240 {
    for c in 0..320 {
      DrawPixel(160 + c, 120 + r, GetColor(255 - c / 2, 192 - r / 2, 32));
    }
  }
  SetMainWindowText("click or hit any key...\0".as_ptr());
  ScreenFlip();
  WaitKey();

  SetDrawScreen(DX_SCREEN_FRONT);
  SetMainWindowText("end\0".as_ptr());
  WaitTimer(1000);

  DeleteMusicMem(mh);

  DxLib_End();
}
}

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  // use super::*;
  use crate::dum_screen;

  /// test screen
  #[test]
  fn test_screen() {
//    assert_eq!(dum_screen(), ());
  }
}
