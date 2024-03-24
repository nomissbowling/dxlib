// Pixel Shader
struct PS_INPUT {
  float4 dif : COLOR0; // from Vertex Shader
  float2 texCoords0 : TEXCOORD0; // from Vertex Shader
};

struct PS_OUTPUT {
  float4 color0 : SV_TARGET0; // screen pixel color
};

struct DX_D3D11_PS_CONST_BUFFER_BASE {
  float4 FactorColor; // alpha etc
  float MulAlphaColor; // 0.0f: ignore 1.0f: mul alpha
  float AlphaTestRef; // alpha test compare with it
  float2 Padding1;
  int AlphaTestCmpMode; // alpha test mode (DX_CMP_NEVER etc)
  int3 Padding2;
  float4 IgnoreTextureColor; // color when ignore texture
};

cbuffer cbD3D11_CONST_BUFFER_PS_BASE : register(b1) {
  DX_D3D11_PS_CONST_BUFFER_BASE g_Base;
};

SamplerState g_DiffuseMapSampler : register(s0);
Texture2D g_DiffuseMapTexture : register(t0);

PS_OUTPUT main(PS_INPUT psi)
{
  PS_OUTPUT pso;
  float4 dc; // texture diffused color

  dc = g_DiffuseMapTexture.Sample(g_DiffuseMapSampler, psi.texCoords0);
  pso.color0 = dc * psi.dif;
  return pso;
}
