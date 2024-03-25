//! dx bridge for DxLib
//!
//! tested with the version DxLibDotNet3_24b.zip
//!
//! must use "for C# DxLib_x64.dll" (no_mangle for C)
//!
//! - download "for C# DxLib_x64.dll" from https://dxlib.xsrv.jp/
//! - dumpbin /exports DxLib_x64.dll &gt; DxLib_x64.def
//! - edit DxLib_x64.def ( see also sample etc/DxLib_x64.def )
//! - lib /machine:x64 /def:DxLib_x64.def /out:DxLib_x64.dll.lib
//! - copy DxLib_x64.dll.lib .
//!
//! ( see also https://docs.rs/crate/dxlib/0.0.1/source/bin/ )
//!
//! make bridge "for R DxLib_x64_R.dll" it calls DxLib_x64.dll
//!
//! - edit DxLib_x64_R.def ( see also sample etc/DxLib_x64_R.def )
//! - link /machine:x64 /def:DxLib_x64_R.def /noentry /dll /out:DxLib_x64_R.dll /implib:DxLib_x64_R.dll.lib
//! - copy DxLib_x64_R.dll.lib .
//! - copy DxLib_x64_R.dll .
//!
//! ( see also https://docs.rs/crate/dxlib/0.0.1/source/bin/ )
//!
//! compile .hlsl to .vso and .pso by ShaderCompiler distributed with DxLib
//!
//! - ShaderCompiler /Tvs_4_0 shader_VS.hlsl
//! - ShaderCompiler /Tps_4_0 shader_PS.hlsl
//!

use std::ffi::{c_void};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VECTOR {
  pub x: f32,
  pub y: f32,
  pub z: f32
}

impl VECTOR {
  pub fn new(x: f32, y: f32, z: f32) -> Self { VECTOR{x, y, z} }
  pub fn zeros() -> Self { VECTOR::new(0.0, 0.0, 0.0) }
  pub fn get(v: &[f32; 3]) -> Self { VECTOR::new(v[0], v[1], v[2]) }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VECTOR_D {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

impl VECTOR_D {
  pub fn new(x: f64, y: f64, z: f64) -> Self { VECTOR_D{x, y, z} }
  pub fn zeros() -> Self { VECTOR_D::new(0.0, 0.0, 0.0) }
  pub fn get(v: &[f64; 3]) -> Self { VECTOR_D::new(v[0], v[1], v[2]) }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct FLOAT2 {
  pub u: f32,
  pub v: f32
}

impl FLOAT2 {
  pub fn new(u: f32, v: f32) -> Self { FLOAT2{u, v} }
  pub fn zeros() -> Self { FLOAT2::new(0.0, 0.0) }
  pub fn get(v: &[f32; 2]) -> Self { FLOAT2::new(v[0], v[1]) }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct COLOR_F {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32
}

impl COLOR_F {
  pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self { COLOR_F{r, g, b, a} }
  pub fn zeros() -> Self { COLOR_F::new(0.0, 0.0, 0.0, 0.0) }
  pub fn get(v: &[f32; 4]) -> Self { COLOR_F::new(v[0], v[1], v[2], v[3]) }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct COLOR_U8 {
  pub b: u8,
  pub g: u8,
  pub r: u8,
  pub a: u8
}

impl COLOR_U8 {
  pub fn new(b: u8, g: u8, r: u8, a: u8) -> Self { COLOR_U8{b, g, r, a} }
  pub fn zeros() -> Self { COLOR_U8::new(0, 0, 0, 0) }
  pub fn get(v: &[u8; 4]) -> Self { COLOR_U8::new(v[2], v[1], v[0], v[3]) }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct FLOAT4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32
}

impl FLOAT4 {
  pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { FLOAT4{x, y, z, w} }
  pub fn zeros() -> Self { FLOAT4::new(0.0, 0.0, 0.0, 0.0) }
  pub fn get(v: &[f32; 4]) -> Self { FLOAT4::new(v[0], v[1], v[2], v[3]) }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct DOUBLE4 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64
}

impl DOUBLE4 {
  pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self { DOUBLE4{x, y, z, w} }
  pub fn zeros() -> Self { DOUBLE4::new(0.0, 0.0, 0.0, 0.0) }
  pub fn get(v: &[f64; 4]) -> Self { DOUBLE4::new(v[0], v[1], v[2], v[3]) }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VERTEX3DSHADER {
  pub pos: VECTOR,
  pub spos: FLOAT4,
  pub norm: VECTOR,
  pub tan: VECTOR,
  pub binorm: VECTOR,
  pub dif: COLOR_U8,
  pub spc: COLOR_U8,
  pub uv: FLOAT2, // u: f32, v: f32
  pub suv: FLOAT2 // su: f32, sv: f32
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct MATRIX {
  pub m: [[f32; 4]; 4]
}

impl MATRIX {
  pub fn new() -> Self {
    MATRIX{m: [
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0]]}
  }
  pub fn zeros() -> Self { MATRIX::new() }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct MATRIX_D {
  pub m: [[f64; 4]; 4]
}

impl MATRIX_D {
  pub fn new() -> Self {
    MATRIX_D{m: [
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0]]}
  }
  pub fn zeros() -> Self { MATRIX_D::new() }
}

// #[no_mangle] // needless

#[link(name="DxLib_x64_R.dll", kind="dylib")]
extern "stdcall" {
  pub fn DxLib_Init() -> i32;
  pub fn DxLib_End() -> i32;
  pub fn ChangeWindowMode(Flag: i32) -> i32;
  // int __stdcall SetGraphMode(int ScreenSizeX, int ScreenSizeY,
  //   int ColorBitDepth, int RefreshRate=60) -> i32;
  pub fn SetGraphMode(ScreenSizeX: i32, ScreenSizeY: i32,
    ColorBitDepth: i32, RefreshRate: i32) -> i32;
  pub fn SetOutApplicationLogValidFlag(b: i32) -> i32;
  pub fn SetMainWindowText(WindowText: *const u8) -> i32;
  pub fn SetDrawBlendMode(bm: i32, pal: i32) -> i32;

  pub fn GetJoypadInputState(inputtype: i32) -> i32;
  pub fn GetHitKeyStateAll(ksbuf: *const u8) -> i32; // 256 bytes
  pub fn WaitKey() -> i32; // call ProcessMessage() fps
  pub fn WaitTimer(msec: i32) -> i32; // call ProcessMessage() fps
  pub fn ProcessMessage() -> i32;

  pub fn ClearDrawScreen(rct: *const c_void) -> i32;
  pub fn SetDrawScreen(ds: i32) -> i32; // DX_SCREEN_FRONT DX_SCREEN_BACK
  pub fn ScreenFlip() -> i32;

  pub fn SelectMidiMode(md: i32) -> i32;
  pub fn InitMusicMem() -> i32;
  pub fn LoadMusicMem(mid: *const u8) -> i32;
  pub fn DeleteMusicMem(mh: i32) -> i32;
  pub fn PlayMusicMem(mh: i32, playtype: i32) -> i32;
  pub fn StopMusicMem(mh: i32) -> i32;
  pub fn ProcessMusicMem() -> i32;
  pub fn SetVolumeMusicMem(volume: i32, mh: i32) -> i32;

  pub fn LoadSoundMem(snd: *const u8) -> i32;
  pub fn DeleteSoundMem(sh: i32, logout: i32) -> i32;
  pub fn PlaySoundMem(sh: i32, playtype: i32, topposition: i32) -> i32;
  pub fn StopSoundMem(sh: i32) -> i32;
  pub fn ChangeVolumeSoundMem(volumepal: i32, sh: i32) -> i32;

  pub fn LoadGraph(fname: *const u8) -> i32;
  pub fn DeleteGraph(gh: i32, logout: i32) -> i32;
  pub fn DrawGraph(x: i32, y: i32, gh: i32, trans: i32) -> i32;
  pub fn DrawRotaGraph(x: i32, y: i32, extrate: f64, angle: f64,
    gh: i32, trans: i32, reversex: i32, reversey: i32) -> i32;

  pub fn InitShader() -> i32;
  pub fn LoadVertexShader(vso: *const u8) -> i32;
  pub fn LoadPixelShader(pso: *const u8) -> i32;
  pub fn LoadGeometryShader(gso: *const u8) -> i32;
  pub fn DeleteShader(h: i32) -> i32;

  pub fn SetUseBackCulling(flg: i32) -> i32;
  pub fn SetUseTextureToShader(stage: i32, gh: i32) -> i32;
  pub fn SetUseVertexShader(vsh: i32) -> i32;
  pub fn SetUsePixelShader(psh: i32) -> i32;
  pub fn SetUseGeometryShader(gsh: i32) -> i32;

  pub fn CreateLookAtMatrix(o: *mut MATRIX,
    eye: *const VECTOR, at: *const VECTOR, up: *const VECTOR) -> i32;
  pub fn SetCameraNearFar(near: f32, far: f32) -> i32;
  pub fn SetCameraViewMatrix(vm: MATRIX) -> i32; // MTranspose (GL<->DX)
  pub fn GetCameraProjectionMatrix() -> MATRIX;
  pub fn GetProjectionMatrix() -> MATRIX;
  pub fn SetTransformToProjection(m: *const MATRIX) -> i32;

  pub fn CreatePerspectiveFovMatrix(m: *mut MATRIX,
    fov: f32, zn: f32, zf: f32, aspect: f32) -> i32; // default aspect -1.0
  pub fn CreateViewportMatrix(m: *mut MATRIX,
    cx: f32, cy: f32, w: f32, h: f32) -> i32;
  pub fn SetTransformToViewport(m: *const MATRIX) -> i32;

  pub fn DrawPolygon3DToShader(va: *const VERTEX3DSHADER, npolygons: i32) -> i32;

  pub fn InitFontToHandle() -> i32;
  pub fn DeleteFontToHandle(fh: i32) -> i32;
  pub fn CreateFontToHandle(font: *const u8, sz: i32, thick: i32,
    fonttype: i32, charset: i32, edgesz: i32, italic: i32, handle: i32) -> i32;
  pub fn LoadFontDataToHandle(fname: *const u8, edgesz: i32) -> i32;
  pub fn DrawStringToHandle(x: i32, y: i32, s: *const u8,
    color: u32, fh: i32, edgecolor: u32, vertical_flag: i32) -> i32;
  pub fn DrawFormatStringToHandle(x: i32, y: i32, color: u32, fh: i32,
    fmt: *const u8) -> i32;

  pub fn GetColor(r: i32, g: i32, b: i32) -> u32;
  pub fn DrawPixel(x: i32, y: i32, c: u32) -> i32;
}

pub const NULL: *const c_void = 0 as *const c_void;
pub const FALSE: i32 = 0;
pub const TRUE: i32 = 1;

pub const DX_SCREEN_BACK: i32 = -2;
pub const DX_SCREEN_WORK: i32 = -3;
pub const DX_SCREEN_FRONT: i32 = -4;

pub const DX_MIDIMODE_MCI: i32 = 0;

pub const DX_PLAYTYPE_LOOPBIT: i32 = 2;
pub const DX_PLAYTYPE_BACKBIT: i32 = 1;
pub const DX_PLAYTYPE_NORMAL: i32 = 0;
pub const DX_PLAYTYPE_BACK: i32 = DX_PLAYTYPE_BACKBIT;
pub const DX_PLAYTYPE_LOOP: i32 = DX_PLAYTYPE_LOOPBIT | DX_PLAYTYPE_BACKBIT;
