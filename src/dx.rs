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
pub struct CUBEDATA {
  pub p0: VECTOR,
  pub p1: VECTOR,
  pub dif: COLOR_U8,
  pub spc: COLOR_U8
}

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

#[derive(Debug, Clone, PartialEq)]
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
  pub fn from_u8(u: &COLOR_U8) -> Self {
    let v = [u.r, u.g, u.b, u.a].iter().map(|&c|
      c as f32 / 255.0).collect::<Vec<_>>();
    COLOR_F::new(v[0], v[1], v[2], v[3])
  }
  pub fn from_float4(p: &FLOAT4) -> Self { // not complemental as_float4
    let v = [p.x, p.y, p.z, p.w].iter().enumerate().map(|(i, &c)|
      if i < 3 { (c + 2.0) / 4.0 } else { 1.0 }).collect::<Vec<_>>();
    COLOR_F::new(v[0], v[1], v[2], v[3])
  }
  pub fn from_u32(u: u32) -> Self {
    COLOR_F::from_u8(&COLOR_U8::from_u32(u))
  }
  pub fn as_float4(&self) -> FLOAT4 { // not complemental from_float4
    unsafe { (*(&self.r as *const f32 as *const FLOAT4)).clone() }
  }
}

#[derive(Debug, Clone, PartialEq)]
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
  pub fn from_f(f: &COLOR_F) -> Self {
    let v = [f.b, f.g, f.r, f.a].iter().map(|&c|
      (c * 255.0) as u8).collect::<Vec<_>>();
    COLOR_U8::new(v[0], v[1], v[2], v[3])
  }
  pub fn from_float4(p: &FLOAT4) -> Self {
    COLOR_U8::from_f(&COLOR_F::from_float4(p))
  }
  pub fn from_u32(u: u32) -> Self {
    unsafe { (*(&u as *const u32 as *const COLOR_U8)).clone() }
  }
  pub fn as_u32(&self) -> u32 {
    unsafe { *(&self.b as *const u8 as *const u32) }
  }
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
pub struct VERTEX3D {
  pub pos: VECTOR,
  pub norm: VECTOR,
  pub dif: COLOR_U8,
  pub spc: COLOR_U8,
  pub uv: FLOAT2, // u: f32, v: f32
  pub suv: FLOAT2 // su: f32, sv: f32
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
pub struct LIGHTPARAM {
  pub light_type: i32,
  pub diffuse: COLOR_F,
  pub specular: COLOR_F,
  pub ambient: COLOR_F,
  pub position: VECTOR,
  pub direction: VECTOR,
  pub range: f32,
  pub fall_off: f32,
  pub attenuation0: f32,
  pub attenuation1: f32,
  pub attenuation2: f32,
  pub theta: f32,
  pub phi: f32
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct MATERIALPARAM {
  pub diffuse: COLOR_F,
  pub ambient: COLOR_F,
  pub specular: COLOR_F,
  pub emissive: COLOR_F,
  pub power: f32
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
  pub fn identity() -> Self { MATRIX::new() }
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
  pub fn identity() -> Self { MATRIX_D::new() }
}

// #[no_mangle] // needless

#[link(name="DxLib_x64_R.dll", kind="dylib")]
extern "stdcall" {
  pub fn SetUseNormalDrawShader(flg: i32) -> i32; // flg=TRUE
  pub fn SetUseSoftwareRenderModeFlag(flg: i32) -> i32; // flg=FALSE

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
  pub fn CheckHitKeyAll(typ: i32) -> i32; // DX_CHECKINPUT_ALL
  pub fn CheckHitKey(code: i32) -> i32;
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

  pub fn MakeScreen(xsz: i32, ysz: i32, trans: i32) -> i32;
  pub fn GetGraphSize(gh: i32, xsz: *mut i32, ysz: *mut i32) -> i32;
  pub fn GetDrawScreenGraph(l: i32, t: i32, r: i32, b: i32,
    gh: i32, use_client_flag: i32) -> i32;
  pub fn MakeGraph(xsz: i32, ysz: i32, not_use_3d_flag: i32) -> i32;
  pub fn LoadDivGraph(fname: *const u8, allnum: i32,
    xnum: i32, ynum: i32, xsz: i32, ysz: i32, handle_buf: *mut i32,
    not_use_3d_flag: i32, xstride: i32, ystride: i32) -> i32;
  pub fn LoadGraph(fname: *const u8) -> i32;
  pub fn DeleteGraph(gh: i32, logout: i32) -> i32;
  pub fn DrawGraph(x: i32, y: i32, gh: i32, trans: i32) -> i32;
  pub fn DrawTurnGraph(x: i32, y: i32, gh: i32, trans: i32) -> i32;
  pub fn DrawExtendGraph(l: i32, t: i32, r: i32, b: i32,
    gh: i32, trans: i32) -> i32;
  pub fn DrawRotaGraph(x: i32, y: i32, extrate: f64, angle: f64,
    gh: i32, trans: i32, reversex: i32, reversey: i32) -> i32;
  pub fn DrawModiGraph(xlt: i32, ylt: i32, xrt: i32, yrt: i32,
    xrb: i32, yrb: i32, xlb: i32, ylb: i32, gh: i32, trans: i32) -> i32;
  pub fn DrawRectGraph(x: i32, y: i32, srcx: i32, srcy: i32, w: i32, h: i32,
    gh: i32, trans: i32, reversex: i32, reversey: i32) -> i32;
  pub fn DrawRectExtendGraph(l: i32, t: i32, r: i32, b: i32,
    srcx: i32, srcy: i32, w: i32, h: i32, gh: i32, trans: i32) -> i32;

  pub fn InitShader() -> i32;
  pub fn LoadVertexShader(vso: *const u8) -> i32;
  pub fn LoadPixelShader(pso: *const u8) -> i32;
  pub fn LoadGeometryShader(gso: *const u8) -> i32;
  pub fn DeleteShader(h: i32) -> i32;

  pub fn CreateDirLightHandle(d: VECTOR) -> i32;
  pub fn CreateSpotLightHandle(p: VECTOR, d: VECTOR, oa: f32, ia: f32,
    rng: f32, a0: f32, a1: f32, a2: f32) -> i32;
  pub fn CreatePointLightHandle(p: VECTOR,
    rng: f32, a0: f32, a1: f32, a2: f32) -> i32;
  pub fn SetLightTypeHandle(lh: i32, typ: i32) -> i32;
  pub fn SetLightEnableHandle(lh: i32, flg: i32) -> i32;
  pub fn SetLightDifColorHandle(lh: i32, c: COLOR_F) -> i32;
  pub fn SetLightSpcColorHandle(lh: i32, c: COLOR_F) -> i32;
  pub fn SetLightAmbColorHandle(lh: i32, c: COLOR_F) -> i32;
  pub fn SetLightDirectionHandle(lh: i32, d: VECTOR) -> i32;
  pub fn SetLightPositionHandle(lh: i32, p: VECTOR) -> i32;
  pub fn SetLightRangeAttenHandle(lh: i32,
    rng: f32, a0: f32, a1: f32, a2: f32) -> i32;
  pub fn SetLightAngleHandle(lh: i32, oa: f32, ia: f32) -> i32;
  pub fn SetLightUseShadowMapHandle(lh: i32, ssi: i32, flg: i32) -> i32;
  pub fn GetLightTypeHandle(lh: i32) -> i32; // DX_LIGHTTYPE_DIRECTIONAL etc
  pub fn GetLightEnableHandle(lh: i32) -> i32;
  pub fn GetLightDifColorHandle(lh: i32) -> COLOR_F;
  pub fn GetLightSpcColorHandle(lh: i32) -> COLOR_F;
  pub fn GetLightAmbColorHandle(lh: i32) -> COLOR_F;
  pub fn GetLightDirectionHandle(lh: i32) -> VECTOR;
  pub fn GetLightPositionHandle(lh: i32) -> VECTOR;
  pub fn GetLightRangeAttenHandle(lh: i32,
    rng: *mut f32, a0: *mut f32, a1: *mut f32, a2: *mut f32) -> i32;
  pub fn GetLightAngleHandle(lh: i32, oa: *mut f32, ia: *mut f32) -> i32;
  pub fn GetEnableLightHandleNum() -> i32;
  pub fn GetEnableLightHandle(i: i32) -> i32;
  pub fn DeleteLightHandle(lh: i32) -> i32;
  pub fn DeleteLightHandleAll() -> i32;

  pub fn SetMaterialUseVertDifColor(flg: i32) -> i32; // default TRUE
  pub fn SetMaterialUseVertSpcColor(flg: i32) -> i32; // default TRUE
  pub fn SetMaterialParam(mp: MATERIALPARAM) -> i32;
  pub fn SetUseLighting(flg: i32) -> i32; // default TRUE
  pub fn SetUseSpecular(flg: i32) -> i32; // default TRUE
  pub fn SetGlobalAmbientLight(c: COLOR_F) -> i32; // 0 0 0 0 or .2 .2 .2 .2
  pub fn SetUseLightAngleAttenuation(flg: i32) -> i32; // default TRUE

  pub fn SetLightEnable(flg: i32) -> i32; // default TRUE
  pub fn SetLightDifColor(c: COLOR_F) -> i32; // 1 1 1 1
  pub fn SetLightSpcColor(c: COLOR_F) -> i32; // 1 1 1 1 or .5 .5 .5 .5
  pub fn SetLightAmbColor(c: COLOR_F) -> i32; // .33 .33 .33 .33
  pub fn SetLightDirection(d: VECTOR) -> i32; // 1 -1 1 (variable)
  pub fn SetLightPosition(p: VECTOR) -> i32; // no effect to directional light
  pub fn SetLightRangeAtten(rng: f32, a0: f32, a1: f32, a2: f32) -> i32;
  pub fn SetLightAngle(oa: f32, ia: f32) -> i32; // oa 0-DX_PI_F ia 0-oa
  pub fn SetLightUseShadowMap(ssi: i32, flg: i32) -> i32;

  pub fn InitShaderConstantBuffer() -> i32; // DX11
  pub fn CreateShaderConstantBuffer(sz: i32) -> i32; // DX11 n * 4 * sizeof f32
  pub fn DeleteShaderConstantBuffer(cbh: i32) -> i32; // DX11
  pub fn GetBufferShaderConstantBuffer(cbh: i32) -> *mut FLOAT4; // DX11
  pub fn UpdateShaderConstantBuffer(cbh: i32) -> i32; // DX11
  pub fn SetShaderConstantBuffer(cbh: i32, ts: i32, slot: i32) -> i32; // DX
  // ts: DX_SHADERTYPE_VERTEX DX_SHADERTYPE_PIXEL etc

  pub fn GetConstDefaultParamFToShader(
    n: *const u8, sh: i32) -> *const FLOAT4; // DX9
  pub fn GetConstDefaultParamFToShaderWithStrLen(
    n: *const u8, l: usize, sh: i32) -> *const FLOAT4; // DX9
  pub fn GetConstIndexToShader(
    n: *const u8, sh: i32) -> i32; // DX9
  pub fn GetConstIndexToShaderWithStrLen(
    n: *const u8, l: usize, sh: i32) -> i32; // DX9
  pub fn GetConstCountToShader(
    n: *const u8, sh: i32) -> i32; // DX9
  pub fn GetConstCountToShaderWithStrLen(
    n: *const u8, l: usize, sh: i32) -> i32; // DX9
  pub fn SetVSConstF(i: i32, p: FLOAT4) -> i32; // DX9
  pub fn SetPSConstF(i: i32, p: FLOAT4) -> i32; // DX9

  pub fn SetUseBackCulling(flg: i32) -> i32;
  pub fn SetRenderTargetToShader(target_index: i32, draw_screen: i32,
    surface_index: i32, mip_level: i32) -> i32; // surface_index=0, mip_level=0
  pub fn SetUseTextureToShader(stage: i32, gh: i32) -> i32;
  pub fn SetUseVertexShader(vsh: i32) -> i32;
  pub fn SetUsePixelShader(psh: i32) -> i32;
  pub fn SetUseGeometryShader(gsh: i32) -> i32;

  pub fn SetUseZBufferFlag(flg: i32) -> i32; // flg=FALSE (2D 3D)
  pub fn SetWriteZBufferFlag(flg: i32) -> i32; // flg=FALSE (2D 3D)
  pub fn SetUseZBuffer3D(flg: i32) -> i32; // flg=FALSE (3D)
  pub fn SetWriteZBuffer3D(flg: i32) -> i32; // flg=FALSE (3D)
  pub fn SetDrawZ(z: f32) -> i32; // z=0.2 (2D)

  pub fn CreateLookAtMatrix(o: *mut MATRIX,
    eye: *const VECTOR, at: *const VECTOR, up: *const VECTOR) -> i32;
  pub fn SetCameraNearFar(near: f32, far: f32) -> i32;
  pub fn SetCameraViewMatrix(vm: MATRIX) -> i32; // MTranspose (GL<->DX)
  pub fn GetCameraProjectionMatrix() -> MATRIX;
  pub fn GetTransformToProjectionMatrix(m: *mut MATRIX) -> i32;
  pub fn SetTransformToProjection(m: *const MATRIX) -> i32;

  pub fn CreatePerspectiveFovMatrix(m: *mut MATRIX,
    fov: f32, zn: f32, zf: f32, aspect: f32) -> i32; // default aspect -1.0
  pub fn CreateViewportMatrix(m: *mut MATRIX,
    cx: f32, cy: f32, w: f32, h: f32) -> i32;
  pub fn SetTransformToViewport(m: *const MATRIX) -> i32;

  pub fn DrawPolygon3DToShader(va: *const VERTEX3DSHADER, npolygons: i32) -> i32;
  pub fn DrawPolygon3D(va: *const VERTEX3D, npolygons: i32,
    gh: i32, trans: i32) -> i32;

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
  pub fn DrawBox(l: i32, t: i32, r: i32, b: i32, c: u32, fill: i32) -> i32;

  pub fn DrawPixel3D(p: VECTOR, c: u32) -> i32;
  pub fn DrawLine3D(s: VECTOR, e: VECTOR, c: u32) -> i32;
  pub fn DrawTriangle3D(p0: VECTOR, p1: VECTOR, p2: VECTOR,
    c: u32, fill: i32) -> i32;
  pub fn DrawCube3D(p0: VECTOR, p1: VECTOR,
    dif: u32, spc: u32, fill: i32) -> i32;
  pub fn DrawCubeSet3D(cube_array: *const CUBEDATA, n: i32, fill: i32) -> i32;
  pub fn DrawSphere3D(c: VECTOR, r: f32, div_num: i32,
    dif: u32, spc: u32, fill: i32) -> i32;
  pub fn DrawCapsule3D(p0: VECTOR, p1: VECTOR, r: f32, div_num: i32,
    dif: u32, spc: u32, fill: i32) -> i32;
  pub fn DrawCone3D(top: VECTOR, bottom: VECTOR, r: f32, div_num: i32,
    dif: u32, spc: u32, fill: i32) -> i32;

  /// private
  /// - [https://densanken.com/wiki/index.php?dx%A5%E9%A5%A4%A5%D6%A5%E9%A5%EA%B1%A3%A4%B7%B4%D8%BF%F4%A4%CE%A5%DA%A1%BC%A5%B8](https://densanken.com/wiki/index.php?dx%A5%E9%A5%A4%A5%D6%A5%E9%A5%EA%B1%A3%A4%B7%B4%D8%BF%F4%A4%CE%A5%DA%A1%BC%A5%B8)
  pub fn SetWindowStyleMode(s: i32) -> i32;
  pub fn SetUseBackBufferTransColorFlag(f: i32) -> i32;
  pub fn SetUseDirect3DVersion(v: i32) -> i32;
}

pub const NULL: *const c_void = 0 as *const c_void;
pub const FALSE: i32 = 0;
pub const TRUE: i32 = 1;

pub const DX_SCREEN_BACK: i32 = -2;
pub const DX_SCREEN_WORK: i32 = -3;
pub const DX_SCREEN_FRONT: i32 = -4;

pub const DX_BLENDMODE_NOBLEND: i32 = 0;
pub const DX_BLENDMODE_ALPHA: i32 = 1;
pub const DX_BLENDMODE_ADD: i32 = 2;
pub const DX_BLENDMODE_SUB: i32 = 3;
pub const DX_BLENDMODE_MUL: i32 = 4;
pub const DX_BLENDMODE_XOR: i32 = 6;
pub const DX_BLENDMODE_DESTCOLOR: i32 = 8;
pub const DX_BLENDMODE_INVDESTCOLOR: i32 = 9;
pub const DX_BLENDMODE_INVSRC: i32 = 10;
pub const DX_BLENDMODE_MULA: i32 = 11;
pub const DX_BLENDMODE_SRCCOLOR: i32 = 14;
pub const DX_BLENDMODE_PMA_ALPHA: i32 = 17;
pub const DX_BLENDMODE_PMA_ADD: i32 = 18;
pub const DX_BLENDMODE_PMA_SUB: i32 = 19;
pub const DX_BLENDMODE_PMA_INVSRC: i32 = 20;
pub const DX_BLENDMODE_CUSTOM: i32 = 32;
/// more blend modes
pub const DX_BLENDMODE_NUM: i32 = 33;

pub const DX_MIDIMODE_MCI: i32 = 0;

pub const DX_PLAYTYPE_LOOPBIT: i32 = 2;
pub const DX_PLAYTYPE_BACKBIT: i32 = 1;
pub const DX_PLAYTYPE_NORMAL: i32 = 0;
pub const DX_PLAYTYPE_BACK: i32 = DX_PLAYTYPE_BACKBIT;
pub const DX_PLAYTYPE_LOOP: i32 = DX_PLAYTYPE_LOOPBIT | DX_PLAYTYPE_BACKBIT;

pub const DX_DIRECT3D_NONE: i32 = 0;
pub const DX_DIRECT3D_9: i32 = 1;
pub const DX_DIRECT3D_9EX: i32 = 2;
pub const DX_DIRECT3D_11: i32 = 3;

pub const DX_DIRECT3D_11_FEATURE_LEVEL_9_1: i32 = 0x9100;
pub const DX_DIRECT3D_11_FEATURE_LEVEL_9_2: i32 = 0x9200;
pub const DX_DIRECT3D_11_FEATURE_LEVEL_9_3: i32 = 0x9300;
pub const DX_DIRECT3D_11_FEATURE_LEVEL_10_0: i32 = 0xa000;
pub const DX_DIRECT3D_11_FEATURE_LEVEL_10_1: i32 = 0xa100;
pub const DX_DIRECT3D_11_FEATURE_LEVEL_11_0: i32 = 0xb000;
pub const DX_DIRECT3D_11_FEATURE_LEVEL_11_1: i32 = 0xb100;

pub const DX_SHADERTYPE_VERTEX: i32 = 0;
pub const DX_SHADERTYPE_PIXEL: i32 = 1;
pub const DX_SHADERTYPE_GEOMETRY: i32 = 2;
pub const DX_SHADERTYPE_COMPUTE: i32 = 3;
pub const DX_SHADERTYPE_DOMAIN: i32 = 4;
pub const DX_SHADERTYPE_HULL: i32 = 5;

pub const DX_LIGHTTYPE_D3DLIGHT_POINT: i32 = 1; // D_D3DLIGHT_POINT
pub const DX_LIGHTTYPE_D3DLIGHT_SPOT: i32 = 2; // D_D3DLIGHT_SPOT
pub const DX_LIGHTTYPE_D3DLIGHT_DIRECTIONAL: i32 = 3; // D_D3DLIGHT_DIRECTIONAL
pub const DX_LIGHTTYPE_POINT: i32 = 1; // D_D3DLIGHT_POINT
pub const DX_LIGHTTYPE_SPOT: i32 = 2; // D_D3DLIGHT_SPOT
pub const DX_LIGHTTYPE_DIRECTIONAL: i32 = 3; // D_D3DLIGHT_DIRECTIONAL
