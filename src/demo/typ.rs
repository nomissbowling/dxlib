//! typ demo dx bridge for DxLib
//!

use std::error::Error;
use std::path::PathBuf;

use crate::{dx::*, ext::*, ext::tdx::*, demo};

pub fn proc_sh(sh: &impl Ts, ns: &[&str]) {
  for n in ns {
    let cnt = sh.get_const_count_to_shader(n);
    let arr = sh.get_const_default_param_f_to_shader(n);
    let idx = sh.get_const_index_to_shader(n);
    println!("sh {}: {}, {:?}, {}", n, cnt, arr, idx);
  }
}

pub fn screen(p: &str) -> Result<(), Box<dyn Error>> {
  let wf = false; // true: wire frame, false: surface
  let tex_mode = true; // true: texture color, false: vertex color
  let vert = demo::gen_vert();
  let vgl = from_vts_gl(&demo::gen_vert_gl(),
    &POS::new(0.0, 64.0, 0.0, 1.0), 128.0, tex_mode);
  let pgl = from_vts_gl(&demo::gen_poly_gl(5),
    &POS::new(0.0, -64.0, -224.0, 1.0), 64.0, tex_mode);
  let agl = from_vec_vts_gl(&demo::gen_any_face(),
    &POS::new(128.0, 96.0, 96.0, 1.0), 64.0, false); // always vertex color
  let vss = from_vec_vts_gl(&demo::gen_vec_vts_gl(),
    &POS::new(0.0, 0.0, 0.0, 1.0), 128.0, tex_mode);
  let c6f = from_vec_vts_gl(&demo::gen_c6f_gl(),
    &POS::new(160.0, 160.0, -128.0, 1.0), 40.0, tex_mode)
    .into_iter().flat_map(|v| v).collect(); // cube 6 faces on the one texture

  let tf = false; // true: on the one texture, false: texture each face
  let icosa = from_vec_vec_vts_gl(&demo::gen_icosahedron(tf),
    &POS::new(192.0, 32.0, -96.0, 1.0), 32.0, tex_mode);
  let dodeca = from_vec_vec_vts_gl(&demo::gen_dodecahedron(tf),
    &POS::new(192.0, -80.0, -96.0, 1.0), 32.0, tex_mode);
  let dodeca_center = from_vec_vec_vts_gl(&demo::gen_dodecahedron_center(tf),
    &POS::new(128.0, -80.0, -192.0, 1.0), 32.0, tex_mode);
  let c60 = from_vec_vec_vts_gl(&demo::gen_c60(tf),
    &POS::new(192.0, -192.0, -96.0, 1.0), 32.0, tex_mode);
  let c60_center = from_vec_vec_vts_gl(&demo::gen_c60_center(tf),
    &POS::new(128.0, -192.0, -192.0, 1.0), 32.0, tex_mode);

  let col = [
    get_color(0, 0, 0),
    get_color(255, 0, 0),
    get_color(0, 255, 0),
    get_color(255, 255, 0),
    get_color(0, 0, 255),
    get_color(255, 0, 255),
    get_color(0, 255, 255),
    get_color(255, 255, 255)];
  let amb = COLOR_F::get(&[0.33, 0.33, 0.33, 0.33]);

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
    "_img_256x256_16x64x64.png\0", // 12 scenes
    "_img_64x64_64x8x8.png\0", // 64 scenes
    "_img_256x256_6x64x64.png\0" // 6 faces on the one texture
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

  let lights = vec![
    light::LightParamSub::new(DX_LIGHTTYPE_DIRECTIONAL, // default light
      COLOR_F::from_u32(col[7]), COLOR_F::from_u32(col[7]), amb.clone(),
      VECTOR::get(&[-1.0, 1.0, -1.0]), VECTOR::get(&[1.0, -1.0, 1.0])),
    light::LightParamSub::new(DX_LIGHTTYPE_DIRECTIONAL,
      COLOR_F::from_u32(col[2]), COLOR_F::from_u32(col[2]), amb.clone(),
      VECTOR::get(&[0.0, -512.0, 512.0]), VECTOR::new(0.0, 1.0, -1.0)),
    light::LightParamSub::new(DX_LIGHTTYPE_DIRECTIONAL,
      COLOR_F::from_u32(col[6]), COLOR_F::from_u32(col[6]), amb.clone(),
      VECTOR::get(&[512.0, -512.0, 0.0]), VECTOR::new(-1.0, 1.0, 0.0)),
    light::LightParamSub::new(DX_LIGHTTYPE_DIRECTIONAL,
      COLOR_F::from_u32(col[5]), COLOR_F::from_u32(col[5]), amb.clone(),
      VECTOR::get(&[0.0, 512.0, 0.0]), VECTOR::new(0.0, -1.0, 0.0))];
  let ls = (1..lights.len()).into_iter().map(|k| { // starts from 1 (0 default)
    // if lights[k].light_type == DX_LIGHTTYPE_DIRECTIONAL {} // TODO: skip
    let lt = dx.create_dir_light(lights[k].direction.clone()); // change later
    println!("light[{}]: {:08x}", k, lt.handle());
    lt.set_enable(TRUE);
    lt.set_dif_color(lights[k].diffuse.clone());
    lt.set_spc_color(lights[k].specular.clone());
    lt.set_amb_color(lights[k].ambient.clone());
    lt.set_position(lights[k].position.clone()); // not direction
    lt.set_direction(lights[k].direction.clone()); // after construct
    lt
  }).collect::<Vec<_>>();
  println!("lights: {} + 1", ls.len());

  init_font_to_handle();
  let fsys = dx.create_font("Arial\0", 32, 1, -1, -1, -1, TRUE); // italic
  let fdat = dx.load_font(&res[9]);
  println!("fsys: {:08x} fdat: {:08x}", fsys.handle(), fdat.handle());
  let ani = dx.load_div_graph(&res[10], 12, 4, 3, 64, 64, FALSE, 0, 0);
  // for a in ani.iter() { println!("ani: {:08x}", a.handle()); }
  let blk = dx.load_div_graph(&res[11], 8, 1, 8, 8, 8, FALSE, 8, 0);
  // for b in blk.iter() { println!("blk: {:08x}", b.handle()); }
  let bls = dx.make_graphs_from_div_graph(&blk, TRUE, TRUE, FALSE); // shader
  let t6f = dx.load_graph(&res[12]); // shader 6 faces on the one texture
  let gds = dx.make_graph(64, 64, FALSE); // empty for clipping
  let twh = dx.make_graph_color(64, 64, get_color(255, 255, 255),
    TRUE, TRUE, FALSE); // white texture (through vertex color)

  select_midi_mode(DX_MIDIMODE_MCI);
  bgm.volume(96);
  bgm.play(DX_PLAYTYPE_BACK);

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
    if check_hit_key(0x01) != 0 { break; } // KEY_INPUT_ESCAPE
    if check_hit_key(0x10) != 0 { break; } // KEY_INPUT_Q
    clear_draw_screen(NULL);
    set_use_z_buffer_3d(TRUE);
    set_write_z_buffer_3d(TRUE);
    // set_use_z_buffer_flag(TRUE);
    // set_write_z_buffer_flag(TRUE);
    // set_draw_z(0.2);
    set_draw_blend_mode(DX_BLENDMODE_NOBLEND, 0); // not for shader
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

    let g = dx.make_graph(480, 360, FALSE); // must unreg inner allocation
    g.get_draw_screen(80, 60, 560, 420, TRUE);
    g.draw_rota(0, 0, 0.5, 0.0, TRUE, TRUE, TRUE);
    dx.unreg(Box::new(g));

    let g = dx.get_graph(80, 60, 64, 64, TRUE, FALSE); // unreg inner
    g.draw(160, 0, TRUE);
    dx.unreg(Box::new(g));

    set_draw_screen(DX_SCREEN_WORK);
    set_use_lighting(TRUE); // default TRUE
    set_use_specular(TRUE); // default TRUE
    set_global_ambient_light(COLOR_F::from_u32(col[5]));
    set_use_light_angle_attenuation(TRUE); // default TRUE
    set_light_enable(TRUE); // default TRUE
    set_light_dif_color(lights[0].diffuse.clone());
    set_light_spc_color(lights[0].specular.clone());
    set_light_amb_color(lights[0].ambient.clone());
    // set_light_position(lights[0].position.clone()); // not direction
    set_light_direction(lights[0].direction.clone());
    // set_light_range_atten(1000.0, 1.0, 0.5, 0.25);
    // set_light_angle(2.0 * pi / 3.0, pi / 2.0);
    // set_light_use_shadow_map(ssi, TRUE);
    set_use_back_culling(TRUE); // small true is not same as 1 or TRUE
    // tex.set_to_shader(0); // single texture
    // [&grp, &tex][anim % 2].set_to_shader(0); // changing texture
    // ani[anim % ani.len()].set_to_shader(0); // transparent (black on black)
    // gds.set_to_shader(0); // clipped rect of 2d screen
    shv.set_shader();
    shp.set_shader();
    // shg.set_shader();

    if tick == 0 {
      let nl = dx.get_enable_light_handle_num();
      for l in 0..nl {
        let lh = dx.get_enable_light_handle(l);
        println!("lh[{}] = {:08x}", l, lh);
      }
      proc_sh(&shv, &["g_Reg0\0", "g_Reg1\0", "g_Test\0", "g_Arr\0",
        "g_Common\0", "g_Base\0", "g_OtherMatrix\0", "g_LocalWorldMatrix\0"]);
      proc_sh(&shp, &["g_Reg0\0", "g_Reg1\0", "g_Test\0", "g_Arr\0",
        "g_Common\0", "g_Base\0", "g_ShadowMap\0"]);
    }
    // set_ps_const_f(VecL0, COLOR_F::get(&[1.0, 1.0, 1.0, 1.0]).as_float4());
    // set_ps_const_f(PosL0, COLOR_F::get(&[0.0, 0.0, 0.0, 1.0]).as_float4());

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

    draw_line_3d(VECTOR::zeros(), VECTOR::new(-512.0, 0.0, 0.0), col[7]);
    draw_line_3d(VECTOR::zeros(), VECTOR::new(0.0, -512.0, 0.0), col[7]);
    draw_line_3d(VECTOR::zeros(), VECTOR::new(0.0, 0.0, -512.0), col[7]);
    draw_line_3d(VECTOR::zeros(), VECTOR::new(512.0, 0.0, 0.0), col[1]);
    draw_line_3d(VECTOR::zeros(), VECTOR::new(0.0, 512.0, 0.0), col[2]);
    draw_line_3d(VECTOR::zeros(), VECTOR::new(0.0, 0.0, 512.0), col[4]);

    if tex_mode {
      gds.set_to_shader(0); // clipped rect of 2d screen
    } else {
      twh.set_to_shader(0); // white texture (through vertex color)
    }
    draw_polygon_3d_to_shader_or_wire(&vert, wf);
    draw_polygon_3d_to_shader_or_wire(&vgl, wf);
    draw_polygon_3d_to_shader_or_wire(&pgl, wf);
    for (i, vs) in agl.iter().enumerate() {
      if (tex_mode && i == agl.len() - 1) || tick & 0x00000080 != 0 {
        gds.set_to_shader(0); // clipped rect of 2d screen
      } else {
        twh.set_to_shader(0); // white texture (through vertex color)
      }
      draw_polygon_3d_to_shader_or_wire(vs, wf);
    }
    for (i, vs) in vss.iter().enumerate() {
      if tex_mode {
        if i == 0 {
          gds.set_to_shader(0); // clipped rect of 2d screen
        } else {
          bls[i % bls.len()].set_to_shader(0); // transparent
        }
      } else {
        twh.set_to_shader(0); // white texture (through vertex color)
      }
      draw_polygon_3d_to_shader_or_wire(vs, wf);
    }
    if tex_mode {
      t6f.set_to_shader(0); // 6 faces on the one texture
    } else {
      twh.set_to_shader(0); // white texture (through vertex color)
    }
    draw_polygon_3d_to_shader_or_wire(&c6f, wf);

    for p in [&icosa, &dodeca, &dodeca_center, &c60, &c60_center] {
      for (i, f) in p.iter().enumerate() {
        if tex_mode {
          bls[i % bls.len()].set_to_shader(0); // transparent
        } else {
          twh.set_to_shader(0); // white texture (through vertex color)
        }
        for vs in f.iter() {
          draw_polygon_3d_to_shader_or_wire(vs, wf);
        }
      }
    }

    let ff = if wf { 0 } else { 1 }; // fill flag
    draw_cone_3d(
      VECTOR::new(-192.0, 0.0, -16.0), VECTOR::new(-255.0, 0.0, -16.0),
      32.0, 16, col[3], col[4], ff);
    draw_capsule_3d(
      VECTOR::new(-192.0, -64.0, -16.0), VECTOR::new(-255.0, -64.0, -16.0),
      32.0, 16, col[3], col[4], ff);
    draw_sphere_3d(
      VECTOR::new(-192.0, -128.0, -16.0), 32.0, 16, col[3], col[4], ff);
    draw_cube_3d(
      VECTOR::new(-255.0, -224.0, -48.0), VECTOR::new(-192.0, -160.0, 16.0),
      col[3], col[4], ff);
    let ca = (0..3).into_iter().flat_map(|k|
      (0..3).into_iter().flat_map(|j|
        (0..3).into_iter().map(|i| {
          let r = 8.0;
          let x = -160.0 - i as f32 * r * 3.0;
          let y = -256.0 - j as f32 * r * 3.0;
          let z = 16.0 - k as f32 * r * 3.0;
          CUBEDATA{
            p0: VECTOR::new(x - r, y - r, z - r),
            p1: VECTOR::new(x + r, y + r, z + r),
            dif: COLOR_U8::from_u32(col[3]),
            spc: COLOR_U8::from_u32(col[4])}
        }).collect::<Vec<_>>()).collect::<Vec<_>>()).collect::<Vec<_>>();
    draw_cube_set_3d(&ca, ff);

    grp.draw_turn(320, 0, TRUE);
    grp.draw_extend(0, 480 - 60, 80, 480, TRUE);
    tex.draw_rota(320, 0, 0.5, -m_pi as f64 / 4.0, TRUE, FALSE, FALSE);
    tex.draw_modi(640 - 160, 240 - 60, 640 - 40, 240 - 60,
      640, 240, 640 - 80, 240, TRUE);
    tex.draw_rect(640 - 160, 240 - 120, 32, 32, 64, 64, TRUE, TRUE, TRUE);
    tex.draw_rect_extend(640 - 240, 240 - 180, 640 - 160, 240 - 120,
      32, 32, 64, 64, TRUE);

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
