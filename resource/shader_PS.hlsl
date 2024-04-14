// Pixel Shader
// https://github.com/darknesswind/DxLib/blob/master/DxLibMake/Windows/DxShader_PS_D3D11.h
// https://github.com/darknesswind/DxLib/blob/master/DxLibMake/Shader/Windows/Direct3D11/PixelShader.h

struct PS_INPUT { // from Vertex Shader
  float2 texCoords0 : TEXCOORD0; // through
  float4 dif : COLOR0; // through
  float4 spc : COLOR1; // through
  float3 norm : NORMAL0; // through
  float3 pos : POSITION0; // through
  float4 ppos : SV_POSITION; // pos in projection
};

struct PS_OUTPUT {
  float4 color0 : SV_TARGET0; // screen pixel color
};

#include <shader_common.hlsl>

struct DX_D3D11_PS_CONST_BUFFER_BASE {
  float4 FactorColor; // alpha etc
  float MulAlphaColor; // 0.0f: ignore 1.0f: mul alpha
  float AlphaTestRef; // alpha test compare with it
  float2 Padding1;
  int AlphaTestCmpMode; // alpha test mode (DX_CMP_NEVER etc)
  int NoLightAngleAttenuation; // 0: attenuation, 1: no attenuation
  int UseHalfLambert; // half lambert mode (20240324 later)
  int Padding2;
  float4 IgnoreTextureColor; // color when ignore texture
  float4 DrawAddColor; // add color
};

struct DX_D3D11_PS_CONST_SHADOWMAP {
  float AdjustDepth;
  float GradationParam;
  float Enable_Light0;
  float Enable_Light1;
  float Enable_Light2;
  float3 Padding;
};

struct DX_D3D11_PS_CONST_BUFFER_SHADOWMAP {
  DX_D3D11_PS_CONST_SHADOWMAP Data[3];
};

cbuffer cbD3D11_CONST_BUFFER_COMMON : register(b0) {
  DX_D3D11_CONST_BUFFER_COMMON g_Common;
};

cbuffer cbD3D11_CONST_BUFFER_PS_BASE : register(b1) {
  DX_D3D11_PS_CONST_BUFFER_BASE g_Base;
};

cbuffer cbD3D11_CONST_BUFFER_PS_SHADOWMAP : register(b2) {
  DX_D3D11_PS_CONST_BUFFER_SHADOWMAP g_ShadowMap;
};

// DX_D3D11_PS_CONST_FILTER_SIZE 1280
// cbuffer cbD3D11_CONST_BUFFER_PS_FILTER : register(b3) {
//   DX_D3D11_PS_CONST_BUFFER_FILTER g_Filter;
// };
cbuffer cbD3D11_CONST_BUFFER_PS_FILTER : register(b3) {
  float4 g_Filter[1280 / 4 / 4]; // length will be changed (dummy)
};

SamplerState g_DiffuseMapSampler : register(s0);
Texture2D g_DiffuseMapTexture : register(t0);

cbuffer cb_Test : register(b4) {
float4 g_Test = float4(2.2f, 4.4f, 6.6f, 8.8f);
float4 g_Arr[4] = {
  float4(0.2f, 0.3f, 0.4f, 0.5f),
  float4(0.4f, 0.5f, 0.6f, 0.7f),
  float4(0.6f, 0.7f, 0.8f, 0.9f),
  float4(0.8f, 0.9f, 1.0f, 1.1f)};
};

float4 g_Reg0 : register(c0);
float4 g_Reg1 : register(c1);

cbuffer cb_5 : register(b5) {
  float4 cb_cam_pos4;
};

cbuffer cb_6 : register(b6) {
  float4 cb_a;
  float4 cb_b;
};

cbuffer cb_7 : register(b7) {
  float4 cb_c;
};

struct CamLight {
  float4 cam_pos4;
  float4 cam_lat4;
  float4 r[2]; // light ratio: (r[0].xyzw and r[1].xy), camera angle: r[1].w
};

cbuffer cb_CamLight : register(b8) { // max 14 slots
  CamLight g_CL;
};

struct LIGHT {
  float4 test;
  float4 amb;
  float4 spc;
  float a;
  float3 padding;
};

LIGHT proc_light(PS_INPUT psi, int lh)
{
  float3 n = normalize(psi.norm.xyz);
  float3 look_vec = g_CL.cam_lat4.xyz - g_CL.cam_pos4.xyz;
//  float3 look_vec = float3(0.0f, 0.0f, 0.0f); // test

  DX_D3D11_CONST_LIGHT light = g_Common.Light[lh];
  float4 light_amb = light.Ambient;
  float4 light_pos4;
  light_pos4.xyz = light.Position;
  light_pos4.w = 1.0f;
  float4 light_vec4;
  light_vec4.xyz = light.Direction;
  light_vec4.w = light.RangePow2;

  float4 test = light_vec4;
  test.xyz = light.Specular;
  test.w = 1.0f;

  float3 light_dir = normalize(light_vec4.xyz);
//  float3 light_dir = normalize(float3(1.0f, 1.0f, 1.0f)); // test

  float3 r = -normalize(light_dir);
  float3 v = -normalize(look_vec);
  float2 p = float2(
    max(0.0f, dot(r, n)), // not use camera angle
    dot(v, normalize(dot(r, n) * n))); // both rev. (- * - = +) hide by culling
//  max(0.0f, dot(v, n)) * max(0.0f, dot(r, n))
  float q = g_CL.r[1].w;
  float a = dot(float2(1.0f - q, q), p);
  float4 spc = psi.spc * pow(a, light_vec4.w);
//  float4 spc = psi.spc * pow(a, 1.0f);
  LIGHT l = {test, light_amb, spc, a, float3(0.0f, 0.0f, 0.0f)};
  return l;
}

LIGHT m_s(CamLight e, LIGHT l[6], int i) // mul struct r LIGHT
{
  float r[8]; // not use loop in HLSL
  r[0] = e.r[0].x;
  r[1] = e.r[0].y;
  r[2] = e.r[0].z;
  r[3] = e.r[0].w;
  r[4] = e.r[1].x;
  r[5] = e.r[1].y;
  r[6] = e.r[1].z;
  r[7] = e.r[1].w;
  LIGHT o = {
    float4(0.0f, 0.0f, 0.0f, 0.0f),
    max(0.0f, r[i] * l[i].amb),
    max(0.0f, r[i] * l[i].spc),
    max(0.0f, r[i] * l[i].a),
    float3(0.0f, 0.0f, 0.0f)};
  return o;
}

LIGHT a_s(LIGHT a, LIGHT b) // add struct LIGHT LIGHT
{
  LIGHT o = {
    a.test + b.test,
    a.amb + b.amb,
    a.spc + b.spc,
    a.a + b.a,
    a.padding + b.padding};
  return o;
}

LIGHT c_s(float c, LIGHT a) // clip struct s LIGHT
{
  LIGHT o = {
    min(c, a.test),
    min(c, a.amb),
    min(c, a.spc),
    min(c, a.a),
    min(c, a.padding)};
  return o;
}

LIGHT dot_s(float c, CamLight e, LIGHT l[6]) // dot and clip struct c r LIGHT
{
  return c_s(c, a_s(a_s(a_s(a_s(a_s(
    m_s(e, l, 0), m_s(e, l, 1)), m_s(e, l, 2)),
    m_s(e, l, 3)), m_s(e, l, 4)), m_s(e, l, 5)));
}

PS_OUTPUT main(PS_INPUT psi)
{
  PS_OUTPUT pso;

  LIGHT l[6]; // not use loop in HLSL
  l[0] = proc_light(psi, 0);
  l[1] = proc_light(psi, 1);
  l[2] = proc_light(psi, 2);
  l[3] = proc_light(psi, 3);
  l[4] = proc_light(psi, 4);
  l[5] = proc_light(psi, 5);

  LIGHT o = dot_s(1.0f, g_CL, l);

  // texture diffused color
  float4 dc = g_DiffuseMapTexture.Sample(g_DiffuseMapSampler, psi.texCoords0);
//  pso.color0 = dc * psi.dif; // not use light spc
//  pso.color0 = dc * psi.dif * l[0].a + l[0].spc + l[0].amb; // only light 0
//  pso.color0 = dc * psi.dif * l[1].a + l[1].spc + l[1].amb; // only light 1
//  pso.color0 = (dc * psi.dif * l[1].a + l[1].spc + l[1].amb)
//    * l[0].a + l[0].spc + l[0].amb;
  pso.color0 = dc * psi.dif * o.a + o.spc + o.amb;
//  pso.color0 = l[0].test; // test by light 0 direction or specular
//  pso.color0 = l[1].test; // test by light 1 direction or specular
//  pso.color0 = g_CL.cam_pos4; // test by constant buffer
//  pso.color0 = float4(r[0], r[1], r[2], r[3]); // test by constant buffer
  return pso;
}
