//! typ demo dx bridge for DxLib
//!

use std::error::Error;
use std::path::PathBuf;

use crate::{dx::*, ext::*, ext::tdx::*, demo};

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
    "_font_32_u8_0000.dft\0", // pre convert by CreateDXFontData.exe
    "_img_256x256_16x64x64.png\0" // 12 scenes
  ].into_iter().map(|p|
    base.join(p).to_str().expect("str").to_string()).collect();

  set_out_application_log_valid_flag(FALSE);
  change_window_mode(TRUE); // not full screen
  set_graph_mode(640, 480, 32, 60); // 32 bit 60 fps
  // set_window_style_mode(2); // no frame
  // set_use_back_buffer_trans_color_flag(TRUE); // transparent
  let u8t: &[u8] = &[0x83, 0x65, 0x83, 0x58, 0x83, 0x67, 0x00]; // in cp932
  set_main_window_text_bytes(u8t);

  let mut dx = Tdx::new()?;
  init_music_mem();
  let bgm = dx.load_music_mem(&res[0]);
  let lps = dx.load_sound_mem(&res[2]);
  let snd = dx.load_sound_mem(&res[3]);
  let grp = dx.load_graph(&res[4]);
  let tex = dx.load_graph(&res[5]);
  println!("bgm: {:08x} lps: {:08x} snd: {:08x} grp: {:08x} tex: {:08x}",
    bgm.handle(), lps.handle(), snd.handle(), grp.handle(), tex.handle());
  init_shader();
  let shv = dx.load_vertex_shader(&res[6]);
  let shp = dx.load_pixel_shader(&res[7]);
  let shg = dx.load_geometry_shader(&res[8]);
  println!("shv: {:08x} shp: {:08x} shg: {:08x}",
    shv.handle(), shp.handle(), shg.handle());
  init_font_to_handle();
  let fsys = dx.create_font("Arial\0", 32, 1, -1, -1, -1, TRUE); // italic
  let fdat = dx.load_font(&res[9]);
  println!("fsys: {:08x} fdat: {:08x}", fsys.handle(), fdat.handle());
  let ani = dx.load_div_graph(&res[10], 12, 4, 3, 64, 64, FALSE, 0, 0);
  // for a in ani.iter() { println!("ani: {:08x}", a.handle()); }
  let gds = dx.make_graph(64, 64, FALSE); // empty for clipping

  select_midi_mode(DX_MIDIMODE_MCI);
  bgm.volume(96);
  bgm.play(DX_PLAYTYPE_BACK, TRUE);

  set_main_window_text("loop sound\0");
  lps.volume(96);
  lps.play(DX_PLAYTYPE_LOOP, TRUE);

  set_draw_screen(DX_SCREEN_WORK);
  clear_draw_screen(NULL);
  fsys.draw_string(40, 400, &format!("waiting...\0"),
    get_color(255, 192, 32), get_color(255, 0, 0), FALSE);
  fdat.draw_string(40, 440, &format!("waiting...\0"),
    get_color(32, 192, 255), get_color(0, 0, 255), FALSE);
  screen_flip();
  wait_key();

  set_draw_screen(DX_SCREEN_BACK);
  let m_pi = std::f32::consts::PI;
  let n = 360i32;
  let m = 4;
  for tick in 0..n * m {
    if process_message() != 0 { break; }
    clear_draw_screen(NULL);
    set_draw_blend_mode(DX_BLENDMODE_NOBLEND, 0);
    // loss time test draw many pixel
    for r in 0..360 {
      for c in 0..480 {
        draw_pixel(80 + c, 60 + r, get_color(255 - c / 2, 192 - r / 2, 32));
      }
    }
    let i = tick * 640 / (n * m);
    let anim = (tick >> 6) as usize;
    [&grp, &tex][anim % 2].draw(i * 4 / 8, i * 3 / 8, TRUE); // transparent
    let anim = (tick >> 3) as usize % (2 * ani.len());
    let left = 80;
    let top = 420 - 64 - 13 * anim as i32;
    ani[anim % ani.len()].draw(left, top, TRUE);
    gds.get_draw_screen(left, top, left + 64, top + 64, TRUE); // clipping

    set_draw_screen(DX_SCREEN_WORK);
    set_use_back_culling(TRUE); // small true is not same as 1 or TRUE
    // tex.set_to_shader(0); // single texture
    // [&grp, &tex][anim % 2].set_to_shader(0); // changing texture
    // ani[anim % ani.len()].set_to_shader(0); // transparent (black on black)
    gds.set_to_shader(0); // clipped rect of 2d screen
    shv.set_shader();
    shp.set_shader();
    // shg.set_shader();

    // after set_draw_screen
    let r = 512.0f32;
    let t = tick as f32 * m_pi / 180.0;
    let c = t.cos();
    let s = t.sin();
    let p = (((tick / 4) % 91) - 45) as f32 * m_pi / 180.0;
    let rc = r * p.cos();
    let rs = r * p.sin();
    let cam_pos = VECTOR::new(rc * c, rc * s, rs);
    let cam_lookat = VECTOR::new(0.0, 0.0, 0.0);
    let cam_z = VECTOR::new(0.0, 0.0, 1.0); // 0 0 1
    let mut mv_cam = MATRIX::identity();
    create_look_at_matrix(&mut mv_cam, &cam_pos, &cam_lookat, &cam_z);
    // set_camera_near_far(0.1, 10000.0);
    set_camera_view_matrix(mv_cam); // MTranspose (GL<->DX)
    let mp_cam = get_camera_projection_matrix();
    set_transform_to_projection(&mp_cam);
    // let mut mp = MATRIX::identity();
    // get_transform_to_projection_matrix(&mut mp);
    // create_perspective_fov_matrix(&mut mp, fov, zn, zf, aspect);
    // set_transform_to_projection(&mp);
    // let mut mv = MATRIX::identity();
    // create_viewport_matrix(&mut mv, cx, cy, w, h);
    // set_transform_to_viewport(&mv);

    draw_polygon_3d_to_shader(&vert[0], demo::TPF_Q);
    for i in 0..demo::NFACES_CUBE {
      draw_polygon_3d_to_shader(&vss[i][0], demo::TPF_Q);
    }

    screen_flip();
  }

  set_main_window_text("sound\0");
  snd.volume(128);
  snd.play(DX_PLAYTYPE_NORMAL, TRUE);
  wait_timer(2000);
  set_main_window_text("sound end\0");
  snd.stop();

  Ok(())
}
