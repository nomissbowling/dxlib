// Vertex Shader
// https://github.com/darknesswind/DxLib/blob/master/DxLibMake/Windows/DxShader_VS_D3D11.h
// https://github.com/darknesswind/DxLib/blob/master/DxLibMake/Shader/Windows/Direct3D11/VertexShader.h

struct VS_INPUT {
  float3 pos : POSITION0; // pos in local (VECTOR)
  float4 spos : POSITION1; // sub pos in local (FLOAT4) (skip)
  float3 norm : NORMAL0; // norm in local (VECTOR) (skip)
  float3 tan : TANGENT0; // tangent in local (VECTOR) (skip)
  float3 binorm : BINORMAL0; // bi norm in local (VECTOR) (skip)
  float4 dif : COLOR0; // diffuse (COLOR_U8)
  float4 spc : COLOR1; // specular (COLOR_U8) (skip)
  float2 texCoords0 : TEXCOORD0; // texture UV (FLOAT u, FLOAT v)
  float2 texCoords1 : TEXCOORD1; // sub texture UV (FLOAT su, FLOAT sv) (skip)
};

struct VS_OUTPUT { // to Pixel Shader
  float2 texCoords0 : TEXCOORD0; // through
  float4 dif : COLOR0; // through
  float4 spc : COLOR1; // through
  float3 norm : NORMAL0; // through
  float3 pos : POSITION0; // through
  float4 ppos : SV_POSITION; // pos in projection
};

#include <shader_common.hlsl>

struct DX_D3D11_VS_CONST_BUFFER_BASE {
  float4 AntiViewportMatrix[4];
  float4 ProjectionMatrix[4]; // view -> projection
  float4 ViewMatrix[3]; // world -> view
  float4 LocalWorldMatrix[3]; // local -> world
  float4 ToonOutLineSize;
  float DiffuseSource; // 0.0f: material 1.0f: vertex color
  float SpecularSource; // 0.0f: material 1.0f: vertex color
  float MulSpecularColor; // weight factor on specular color
  float Padding;
};

struct DX_D3D11_VS_CONST_BUFFER_OTHERMATRIX {
  float4 ShadowMapLightViewProjectionMatrix[3][4]; // SMLV x Projection
  float4 TextureMatrix[3][2]; // Matrix for Texture UV
};

struct DX_D3D11_VS_CONST_BUFFER_LOCALWORLDMATRIX {
  float4 Matrix[54 * 3]; // length will be changed
};

cbuffer cbD3D11_CONST_BUFFER_COMMON : register(b0) {
  DX_D3D11_CONST_BUFFER_COMMON g_Common;
};

cbuffer cbD3D11_CONST_BUFFER_VS_BASE : register(b1) {
  DX_D3D11_VS_CONST_BUFFER_BASE g_Base;
};

cbuffer cbD3D11_CONST_BUFFER_VS_OTHERMATRIX : register(b2) {
  DX_D3D11_VS_CONST_BUFFER_OTHERMATRIX g_OtherMatrix;
};

cbuffer cbD3D11_CONST_BUFFER_VS_LOCALWORLDMATRIX : register(b3) {
  DX_D3D11_VS_CONST_BUFFER_LOCALWORLDMATRIX g_LocalWorldMatrix;
};

float4 g_Test = float4(1.1, 2.2, 3.3, 4.4);
float4 g_Arr[4] = {
  float4(0.1, 0.2, 0.3, 0.4),
  float4(0.2, 0.3, 0.4, 0.5),
  float4(0.3, 0.4, 0.5, 0.6),
  float4(0.4, 0.5, 0.6, 0.7)};

VS_OUTPUT main(VS_INPUT vsi)
{
  VS_OUTPUT vso;
  float4 localPosition, worldPosition, viewPosition;

  localPosition.xyz = vsi.pos;
  localPosition.w = 1.0f;
  // local -> world
  worldPosition.x = dot(localPosition, g_Base.LocalWorldMatrix[0]);
  worldPosition.y = dot(localPosition, g_Base.LocalWorldMatrix[1]);
  worldPosition.z = dot(localPosition, g_Base.LocalWorldMatrix[2]);
  worldPosition.w = 1.0f;
  // world -> view
  viewPosition.x = dot(worldPosition, g_Base.ViewMatrix[0]);
  viewPosition.y = dot(worldPosition, g_Base.ViewMatrix[1]);
  viewPosition.z = dot(worldPosition, g_Base.ViewMatrix[2]);
  viewPosition.w = 1.0f;
  // view -> projection
  vso.ppos.x = dot(viewPosition, g_Base.ProjectionMatrix[0]);
  vso.ppos.y = dot(viewPosition, g_Base.ProjectionMatrix[1]);
  vso.ppos.z = dot(viewPosition, g_Base.ProjectionMatrix[2]);
  vso.ppos.w = dot(viewPosition, g_Base.ProjectionMatrix[3]);
  // through parameters
  vso.texCoords0 = vsi.texCoords0;
  vso.dif = vsi.dif;
  vso.spc = vsi.spc;
  vso.norm = vsi.norm;
  vso.pos = vsi.pos;
  return vso;
}
