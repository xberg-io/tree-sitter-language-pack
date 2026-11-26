#define MAX_LIGHTS 4

interface ILight
{
    float3 illuminate(float3 n);
}

struct DirLight : ILight
{
    float3 dir;

    float3 illuminate(float3 n)
    {
        float k = max(dot(n, dir), 0.0);
        return float3(k, k, k);
    }
}

namespace util
{
    float square(float x) { return x * x; }
}

static const float GAMMA = 2.2;
uniform float4 tint;

typedef float3 Color;

enum Mode
{
    Lit,
    Unlit,
}

[shader("fragment")]
float4 fragmentMain(float3 n : NORMAL) : SV_Target
{
    DirLight light;
    float3 c = light.illuminate(n);
    return float4(c * util::square(0.5), 1.0);
}
