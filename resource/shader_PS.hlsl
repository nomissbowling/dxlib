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
  float4 r0;
  float4 r1;
};

cbuffer cb_CamLight : register(b8) { // max 14 slots
  CamLight g_CL;
};

struct LIGHT {
  float4 test;
  float4 amb;
  float4 vec4;
  float a;
  float3 padding;
};

LIGHT proc_light(float3 n, float3 lookat, int lh)
{
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

//  float3 light_dir = normalize(light_vec4.xyz);
//  float3 light_dir = normalize(float3(1.0f, 1.0f, 1.0f)); // test

//  float3 e = normalize(lookat - psi.norm.xyz); // pos ppos
//  float3 e = normalize(lookat);
//  float3 r = normalize(lookat + light_vec4.xyz);
  float3 r = -normalize(lookat + light_vec4.xyz);
//  float3 r = normalize(lookat + light_dir);
//  float3 r = -normalize(lookat + light_dir);
//  float3 r = -normalize(light_dir);
  LIGHT l = {test, light_amb, light_vec4, max(0.0f, dot(r, n)), float3(0.0f, 0.0f, 0.0f)};
  return l;
}

float m_a(float r[6], LIGHT l[6], int i)
{
  return max(0.0f, r[i] * l[i].a);
}

float dot_a(float r[6], LIGHT l[6])
{
  return m_a(r, l, 0) + m_a(r, l, 1) + m_a(r, l, 2)
    + m_a(r, l, 3) + m_a(r, l, 4) + m_a(r, l, 5);
}

float4 m_amb(float r[6], LIGHT l[6], int i)
{
  return max(0.0f, r[i] * l[i].amb);
}

float4 dot_amb(float r[6], LIGHT l[6])
{
  return m_amb(r, l, 0) + m_amb(r, l, 1) + m_amb(r, l, 2)
    + m_amb(r, l, 3) + m_amb(r, l, 4) + m_amb(r, l, 5);
}

float m_s(float r[6], float4 s[6], int i)
{
  return max(0.0f, r[i] * s[i]);
}

float4 dot_s(float r[6], float4 s[6])
{
  return m_s(r, s, 0) + m_s(r, s, 1) + m_s(r, s, 2)
    + m_s(r, s, 3) + m_s(r, s, 4) + m_s(r, s, 5);
}

PS_OUTPUT main(PS_INPUT psi)
{
  PS_OUTPUT pso;

  float3 norm = psi.norm.xyz;
  float3 lat = g_CL.cam_lat4.xyz - g_CL.cam_pos4.xyz;
//  float3 lat = float3(0.0f, 0.0f, 0.0f);

  LIGHT l[6]; // not use loop in HLSL
  l[0] = proc_light(normalize(norm), lat, 0);
  l[1] = proc_light(normalize(norm), lat, 1);
  l[2] = proc_light(normalize(norm), lat, 2);
  l[3] = proc_light(normalize(norm), lat, 3);
  l[4] = proc_light(normalize(norm), lat, 4);
  l[5] = proc_light(normalize(norm), lat, 5);

  float4 s[6]; // not use loop in HLSL
  s[0] = psi.spc * pow(l[0].a, l[0].vec4.w);
//  s[0] = psi.spc * pow(l[0].a, 1.0f);
  s[1] = psi.spc * pow(l[1].a, l[1].vec4.w);
//  s[1] = psi.spc * pow(l[1].a, 1.0f);
  s[2] = psi.spc * pow(l[2].a, l[2].vec4.w);
//  s[2] = psi.spc * pow(l[2].a, 1.0f);
  s[3] = psi.spc * pow(l[3].a, l[3].vec4.w);
//  s[3] = psi.spc * pow(l[3].a, 1.0f);
  s[4] = psi.spc * pow(l[4].a, l[4].vec4.w);
//  s[4] = psi.spc * pow(l[4].a, 1.0f);
  s[5] = psi.spc * pow(l[5].a, l[5].vec4.w);
//  s[5] = psi.spc * pow(l[5].a, 1.0f);

  float r[6]; // not use loop in HLSL
  r[0] = g_CL.r0.x;
  r[1] = g_CL.r0.y;
  r[2] = g_CL.r0.z;
  r[3] = g_CL.r0.w;
  r[4] = g_CL.r1.x;
  r[5] = g_CL.r1.y;
//  float r[6] = {0.8f, 0.8f, 0.8f, 0.8f, 0.0f, 0.0f}; // not use loop in HLSL
  float a = min(1.0f, dot_a(r, l));
  float4 amb = min(1.0f, dot_amb(r, l));
  float4 ss = min(1.0f, dot_s(r, s));

  // texture diffused color
  float4 dc = g_DiffuseMapTexture.Sample(g_DiffuseMapSampler, psi.texCoords0);
//  pso.color0 = dc * psi.dif; // not use light spc
//  pso.color0 = dc * psi.dif * l[0].a + s[0] + l[0].amb; // only light 0
//  pso.color0 = dc * psi.dif * l[1].a + s[1] + l[1].amb; // only light 1
//  pso.color0 = (dc * psi.dif * l[1].a + s[1] + l[1].amb)
//    * l[0].a + s[0] + l[0].amb;
  pso.color0 = dc * psi.dif * a + ss + amb;
//  pso.color0 = test0; // test by light 0 direction or specular
//  pso.color0 = test1; // test by light 1 direction or specular
//  pso.color0 = g_CL.cam_pos4; // test by constant buffer
//  pso.color0 = float4(r[0], r[1], r[2], r[3]); // test by constant buffer
  return pso;
}
