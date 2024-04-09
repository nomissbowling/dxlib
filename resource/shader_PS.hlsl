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
float4 g_Test = float4(2.2, 4.4, 6.6, 8.8);
float4 g_Arr[4] = {
  float4(0.2, 0.3, 0.4, 0.5),
  float4(0.4, 0.5, 0.6, 0.7),
  float4(0.6, 0.7, 0.8, 0.9),
  float4(0.8, 0.9, 1.0, 1.1)};
};

float4 g_Reg0 : register(c0);
float4 g_Reg1 : register(c1);

PS_OUTPUT main(PS_INPUT psi)
{
  PS_OUTPUT pso;

  float4 eye_pos4 = float4(0.0, 0.0, 0.0, 1.0);
  DX_D3D11_CONST_LIGHT light = g_Common.Light[0];
  float4 light_pos4;
  light_pos4.xyz = light.Position;
  light_pos4.w = 1.0;
  float4 light_vec4;
  light_vec4.xyz = light.Direction;
  light_vec4.w = light.RangePow2;

  float3 n = normalize(psi.norm.xyz);
//  float3 light_dir = normalize(light_vec4.xyz);
//  float3 light_dir = normalize(float3(1.0, 1.0, 1.0)); // test
  float3 lookat = eye_pos4.xyz;
//  float3 e = normalize(lookat - psi.norm.xyz); // pos ppos
//  float3 e = normalize(lookat);
//  float3 r = normalize(lookat + light_vec4.xyz);
  float3 r = -normalize(lookat + light_vec4.xyz);
//  float3 r = normalize(lookat + light_dir);
//  float3 r = -normalize(lookat + light_dir);
//  float3 r = -normalize(light_dir);
  float a = dot(r, n);
  float4 s = psi.spc * pow(a, light_vec4.w);
//  float4 s = psi.spc * pow(a, 1.0);

  // texture diffused color
  float4 dc = g_DiffuseMapTexture.Sample(g_DiffuseMapSampler, psi.texCoords0);
//  pso.color0 = dc * psi.dif; // not use light spc
  pso.color0 = dc * psi.dif * a + s;
//  pso.color0 = light_vec4; // test
  return pso;
}
