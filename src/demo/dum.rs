//! dum demo dx bridge for DxLib
//!

use std::path::PathBuf;

use crate::{dx::*, ext::*, demo};

pub fn screen(p: &str) {
  let vert = demo::gen_vert();
  let vts_gl = demo::gen_vts_gl();
  let vss = vss_from_vts_gl(&vts_gl, demo::NFACES_CUBE, demo::VPF_VTS,
    &POS::new(0.0, 0.0, 0.0, 1.0), 128.0, true);

  assert_eq!(vert.len(), demo::VPF_Q);
  assert_eq!(vts_gl.len(), demo::VPF_VTS * demo::NFACES_CUBE);
  assert_eq!(vss.len(), demo::NFACES_CUBE);

  let base = PathBuf::from(p);
  let res: Vec<String> = vec![
    "Fantasie_Impromptu_op66.mid\0",
    "onestop.mid\0",
    "ringout.wav\0",
    "_decision3_.wav\0",
    "_img_320x240_0000.png\0",
    "_texture_128x128_0000.bmp\0",
    "shader_VS.vso\0",
    "shader_PS.pso\0",
    "shader_GS.gso\0",
    "_font_32_u8_0000.dft\0" // pre convert by CreateDXFontData.exe
  ].into_iter().map(|p|
    base.join(p).to_str().expect("str").to_string()).collect();

unsafe {
  SetOutApplicationLogValidFlag(FALSE);
  ChangeWindowMode(TRUE); // not full screen
  SetGraphMode(640, 480, 32, 60); // 32 bit 60 fps
  let u8t: &[u8] = &[0x83, 0x65, 0x83, 0x58, 0x83, 0x67, 0x00]; // in cp932
  SetMainWindowText(u8t.as_ptr());
  if DxLib_Init() == -1 { return; }

  InitMusicMem();
  let mh = LoadMusicMem(res[0].as_ptr());
  let bh = LoadSoundMem(res[2].as_ptr()); // should select long wav
  let sh = LoadSoundMem(res[3].as_ptr()); // should select short wav
  let gh = LoadGraph(res[4].as_ptr());
  let txh = LoadGraph(res[5].as_ptr());
  InitShader();
  let vsh = LoadVertexShader(res[6].as_ptr());
  let psh = LoadPixelShader(res[7].as_ptr());
  // let gsh = LoadGeometryShader(resources[8].as_ptr());
  InitFontToHandle();
/*
  let fh = CreateFontToHandle("Arial\0".as_ptr(), 32, 1,
    -1, -1, -1, TRUE, -1); // only system fonts
*/
  let fh = LoadFontDataToHandle(res[9].as_ptr(), 0); // fixed size italic
  println!("bh: {:08x} sh: {:08x} gh: {:08x} fh: {:08x}", bh, sh, gh, fh);

  SelectMidiMode(DX_MIDIMODE_MCI); // DX_MIDIMODE_DM DX_MIDIMODE_MCI (default)
  SetVolumeMusicMem(96, mh);
  PlayMusicMem(mh, DX_PLAYTYPE_BACK);

  ChangeVolumeSoundMem(128, bh);
  PlaySoundMem(bh, DX_PLAYTYPE_BACK, TRUE);

  SetDrawScreen(DX_SCREEN_WORK);
  ClearDrawScreen(NULL);
  DrawFormatStringToHandle(40, 440, GetColor(32, 192, 255), fh,
    "waiting...\0".as_ptr());
  ScreenFlip();
  SetMainWindowText("click or hit any key...\0".as_ptr());
  WaitKey();

  ChangeVolumeSoundMem(128, sh);
  PlaySoundMem(sh, DX_PLAYTYPE_LOOP, TRUE);

  SetDrawScreen(DX_SCREEN_BACK);
  let m_pi = std::f32::consts::PI;
  let mut tick = 0i32;
  for i in 0..640 {
    if ProcessMessage() != 0 { break; }
    ClearDrawScreen(NULL);
    // loss time test draw many pixel
    for r in 0..240 {
      for c in 0..320 {
        DrawPixel(160 + c, 120 + r, GetColor(255 - c / 2, 192 - r / 2, 32));
      }
    }
    DrawGraph(i * 4 / 8, i * 3 / 8, gh, TRUE); // transparent

    SetDrawScreen(DX_SCREEN_WORK);
    SetUseBackCulling(TRUE); // small true is not same as 1 or TRUE
    SetUseTextureToShader(0, txh);
    SetUseVertexShader(vsh);
    SetUsePixelShader(psh);

    // after SetDrawScreen
    tick += 1;
    let r = 512.0f32;
    let t = tick as f32 * m_pi / 180.0;
    let c = t.cos();
    let s = t.sin();
    let p = ((tick % 91) - 45) as f32 * m_pi / 180.0;
    let rc = r * p.cos();
    let rs = r * p.sin();
    let cam_pos = VECTOR::new(rc * c, rc * s, rs);
    let cam_lookat = VECTOR::new(0.0, 0.0, 0.0);
    let cam_z = VECTOR::new(0.0, 0.0, 1.0); // 0 0 1
    let mut mv_cam = MATRIX::identity();
    CreateLookAtMatrix(&mut mv_cam as *mut MATRIX,
      &cam_pos as *const VECTOR,
      &cam_lookat as *const VECTOR,
      &cam_z as *const VECTOR);
    // SetCameraNearFar(0.1, 10000.0);
    SetCameraViewMatrix(mv_cam); // MTranspose (GL<->DX)
    let mp_cam = GetCameraProjectionMatrix();
    SetTransformToProjection(&mp_cam as *const MATRIX);
    // let mut mp = MATRIX::identity();
    // GetTransformToProjectionMatrix(&mut mp as *mut MATRIX);
    // CreatePerspectiveFovMatrix(&mut mp as *mut MATRIX, fov, zn, zf, aspect);
    // SetTransformToProjection(&mp as *const MATRIX);
    // let mut mv = MATRIX::identity();
    // CreateViewportMatrix(&mut mv as *mut MATRIX, cx, cy, w, h);
    // SetTransformToViewport(&mv as *const MATRIX);

    DrawPolygon3DToShader(&vert[0], vert.len() as i32 / 3);
    for i in 0..demo::NFACES_CUBE {
      DrawPolygon3DToShader(&vss[i][0], vss[i].len() as i32 / 3);
    }

    ScreenFlip();
  }

  StopSoundMem(sh);
  ChangeVolumeSoundMem(255, sh);
  PlaySoundMem(sh, DX_PLAYTYPE_BACK, TRUE);

  SetDrawScreen(DX_SCREEN_FRONT);
  DrawFormatStringToHandle(40, 440, GetColor(32, 192, 255), fh,
    "aBc日本語漢字表示申能utf8\0".as_ptr()); // UTF-8 available on .dft

  let clicked: &str = "clicked\0"; // must terminate NULL
  SetMainWindowText(clicked.as_ptr());
  WaitTimer(1000);
  SetMainWindowText("click\0".as_ptr());
  WaitTimer(1000);
  SetMainWindowText("cli\0".as_ptr());
  WaitTimer(1000);
  SetMainWindowText("c\0".as_ptr());
  WaitTimer(1000);

  StopSoundMem(sh);
  ChangeVolumeSoundMem(128, sh);
  PlaySoundMem(sh, DX_PLAYTYPE_NORMAL, TRUE);

  // ProcessMusicMem();
  // WaitTimer(20000);
  // WaitKey();

  DeleteFontToHandle(fh);
  DeleteShader(psh);
  DeleteShader(vsh);
  DeleteGraph(txh, FALSE);
  DeleteGraph(gh, FALSE);
  DeleteSoundMem(sh, FALSE);
  DeleteSoundMem(bh, FALSE);
  DeleteMusicMem(mh);

  DxLib_End();
}
}
