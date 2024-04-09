// Shader Common
// https://github.com/darknesswind/DxLib/blob/master/DxLibMake/Windows/DxShader_Common_D3D11.h

struct DX_D3D11_CONST_LIGHT {
  int Type; // light type (DX_LIGHTTYPE_POINT etc)
  int3 Padding1;

  float3 Position; // (view)
  float RangePow2;

  float3 Direction; // (view)
  float FallOff; // spot light FallOff

  float3 Diffuse;
  float SpotParam0; // spot light 0 (cos(phi / 2.0f))

  float3 Specular;
  float SpotParam1; // spot light 1 (1.0f / (cos(th / 2.0f) - cos(phi / 2.0f)))

  float4 Ambient; // amb * material amb

  float Attenuation0;
  float Attenuation1;
  float Attenuation2;
  float Padding2;
};

struct DX_D3D11_CONST_MATERIAL {
  float4 Diffuse;
  float4 Specular;
  float4 Ambient_Emissive; // material emissive + material amb * global amb

  float Power; // power of specular
  float3 Padding;
};

struct DX_D3D11_VS_CONST_FOG {
  float LinearAdd; // end / (end - start)
  float LinearDiv; // -1 / (end - start)
  float Density;
  float E; // napier

  float4 Color;
};

struct DX_D3D11_CONST_BUFFER_COMMON {
  DX_D3D11_CONST_LIGHT Light[6]; // length will be changed
  DX_D3D11_CONST_MATERIAL Material;
  DX_D3D11_VS_CONST_FOG Fog;
};
