//! typ demo dx bridge for DxLib
//!

use std::error::Error;
use std::path::PathBuf;

use crate::{dx::*, ext::*, demo};

pub fn screen(p: &str) -> Result<(), Box<dyn Error>> {
  let vert = demo::gen_vert();
  let vts_gl = demo::gen_vts_gl();
  let vss = vss_from_vts_gl(&vts_gl, demo::NFACES_CUBE, demo::VPF_VTS,
    &POS::new(0.0, 0.0, 0.0, 1.0), 128.0);

  assert_eq!(vert.len(), demo::VPF_Q);
  assert_eq!(vts_gl.len(), demo::VPF_VTS * demo::NFACES_CUBE);
  assert_eq!(vss.len(), demo::NFACES_CUBE);

  let base = PathBuf::from(p);
  let res: Vec<String> = vec![
    "Fantasie_Impromptu_op66.mid\0",
    "_decision3_.wav\0",
    "_decision3_.wav\0",
    "_img_320x240_0000.png\0",
    "_texture_128x128_0000.bmp\0",
    "shader_VS.vso\0",
    "shader_PS.pso\0",
    "shader_GS.gso\0",
    "_font_32_u8_0000.dft\0" // pre convert by CreateDXFontData.exe
  ].into_iter().map(|p|
    base.join(p).to_str().expect("str").to_string()).collect();

  Tdx::set_out_application_log_valid_flag(FALSE);
  Tdx::change_window_mode(TRUE); // not full screen
  Tdx::set_graph_mode(640, 480, 32, 60); // 32 bit 60 fps
  let u8t: &[u8] = &[0x83, 0x65, 0x83, 0x58, 0x83, 0x67, 0x00]; // in cp932
  Tdx::set_main_window_text_bytes(u8t);

  let mut dx = Tdx::new()?;
  let sound = dx.load_sound_mem(&res[2]);
  Tdx::wait_timer(1000);
  sound.volume(128);
  sound.play(DX_PLAYTYPE_NORMAL, TRUE);
  Tdx::wait_timer(2000);
  Tdx::set_main_window_text("end\0");
  sound.stop();

  Ok(())
}
