#define MAX_LIGHTS 4
#define SQUARE(x) ((x) * (x))

typedef float4 Color;

float4x4 viewProj;
static const float Ambient = 0.1;
Texture2D albedo : register(t0);
SamplerState linearSampler : register(s0);

enum LightKind
{
    Directional,
    Point
};

struct VSInput
{
    float3 pos : POSITION;
    float2 uv : TEXCOORD0;
};

class PointLight
{
    float3 position;
    float range;

    float attenuate(float d)
    {
        return saturate(1.0 - d / range);
    }

    float3 direction(float3 p);
};

float3 PointLight::direction(float3 p)
{
    return normalize(position - p);
}

namespace Lighting
{
    float lambert(float3 n, float3 l)
    {
        return max(dot(n, l), 0.0);
    }
}

float3 shade(float3 n)
{
    float k = saturate(n.y);
    return float3(k, k, k) + SQUARE(Ambient);
}

[maxvertexcount(3)]
float4 main(VSInput input) : SV_POSITION
{
    float4 c = albedo.Sample(linearSampler, input.uv);
    return mul(float4(shade(input.pos), 1.0), viewProj) * c;
}
