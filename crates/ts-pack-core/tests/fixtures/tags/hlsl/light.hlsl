#define MAX_LIGHTS 4

float4x4 viewProj;

struct VSInput
{
    float3 pos : POSITION;
};

Texture2D albedo : register(t0);

float3 shade(float3 n)
{
    float k = saturate(n.y);
    return float3(k, k, k);
}

[maxvertexcount(3)]
float4 main(VSInput input) : SV_POSITION
{
    return mul(float4(shade(input.pos), 1.0), viewProj);
}
