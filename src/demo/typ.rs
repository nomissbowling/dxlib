//! typ demo dx bridge for DxLib
//!

use std::error::Error;
use std::path::PathBuf;

use crate::{dx::*, ext::*, ext::tdx::Tdx, demo};

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

  Tdx::set_out_application_log_valid_flag(FALSE);
  Tdx::change_window_mode(TRUE); // not full screen
  Tdx::set_graph_mode(640, 480, 32, 60); // 32 bit 60 fps
  let u8t: &[u8] = &[0x83, 0x65, 0x83, 0x58, 0x83, 0x67, 0x00]; // in cp932
  Tdx::set_main_window_text_bytes(u8t);

  let mut dx = Tdx::new()?;
  Tdx::init_music_mem();
  let bgm = dx.load_music_mem(&res[0]);
  let lps = dx.load_sound_mem(&res[2]);
  let snd = dx.load_sound_mem(&res[3]);
  let grp = dx.load_graph(&res[4]);
  let tex = dx.load_graph(&res[5]);
  Tdx::init_shader();

  Tdx::init_font_to_handle();

//  println!("bh: {:08x} sh: {:08x} gh: {:08x} fh: {:08x}", bh, sh, gh, fh);
  println!("bgm: {:08x} lps: {:08x} snd: {:08x} grp: {:08x} tex: {:08x}",
    bgm.handle(), lps.handle(), snd.handle(), grp.handle(), tex.handle());

  Tdx::select_midi_mode(DX_MIDIMODE_MCI);
  bgm.volume(96);
  bgm.play(DX_PLAYTYPE_BACK, TRUE);

  Tdx::set_main_window_text("loop sound\0");
  lps.volume(96);
  lps.play(DX_PLAYTYPE_LOOP, TRUE);

  Tdx::set_draw_screen(DX_SCREEN_WORK);
  Tdx::clear_draw_screen(NULL);
  Tdx::screen_flip();

  for i in 0..640 {
    if Tdx::process_message() != 0 { break; }
    Tdx::clear_draw_screen(NULL);
    // loss time test draw many pixel
    for r in 0..240 {
      for c in 0..320 {
        Tdx::draw_pixel(160 + c, 120 + r,
          Tdx::get_color(255 - c / 2, 192 - r / 2, 32));
      }
    }
    grp.draw(i * 4 / 8, i * 3 / 8, TRUE); // transparent

    Tdx::screen_flip();
  }

  Tdx::set_main_window_text("sound\0");
  snd.volume(128);
  snd.play(DX_PLAYTYPE_NORMAL, TRUE);
  Tdx::wait_timer(2000);
  Tdx::set_main_window_text("sound end\0");
  snd.stop();

  Ok(())
}
