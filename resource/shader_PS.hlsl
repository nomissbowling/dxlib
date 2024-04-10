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
  int3 Padding2;
  float4 IgnoreTextureColor; // color when ignore texture
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
  LIGHT l = {test, light_amb, light_vec4, dot(r, n), float3(0.0f, 0.0f, 0.0f)};
  return l;
}

PS_OUTPUT main(PS_INPUT psi)
{
  PS_OUTPUT pso;

  float4 eye_pos4 = float4(0.0f, 0.0f, 0.0f, 1.0f);

  LIGHT l0 = proc_light(normalize(psi.norm.xyz), eye_pos4.xyz, 0);
  float a0 = l0.a;
  float w0 = l0.vec4.w;
  float4 amb0 = l0.amb;
  float4 test0 = l0.test;

  float4 s0 = psi.spc * pow(a0, w0);
//  float4 s0 = psi.spc * pow(a0, 1.0f);

  LIGHT l1 = proc_light(normalize(psi.norm.xyz), eye_pos4.xyz, 1);
  float a1 = l1.a;
  float w1 = l1.vec4.w;
  float4 amb1 = l1.amb;
  float4 test1 = l1.test;

  float4 s1 = psi.spc * pow(a1, w1);
//  float4 s1 = psi.spc * pow(a1, 1.0f);

  float p = 1.0f;
  float q = 1.0f - p;
  float a = p * a0 + q * a1; // dot(float2(p, q), float2(a0, a1));
  float4 amb = p * amb0 + q * amb1;
  float4 s = p * s0 + q * s1;

  // texture diffused color
  float4 dc = g_DiffuseMapTexture.Sample(g_DiffuseMapSampler, psi.texCoords0);
//  pso.color0 = dc * psi.dif; // not use light spc
//  pso.color0 = dc * psi.dif * a0 + s0 + amb0; // only light 0
//  pso.color0 = dc * psi.dif * a1 + s1 + amb1; // only light 1
//  pso.color0 = (dc * psi.dif * a1 + s1 + amb1) * a0 + s0 + amb0;
  pso.color0 = dc * psi.dif * a + s + amb;
//  pso.color0 = test0; // test by light 0 direction or specular
//  pso.color0 = test1; // test by light 1 direction or specular
  return pso;
}
