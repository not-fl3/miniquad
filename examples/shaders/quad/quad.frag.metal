#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;

struct fragment_function_out
{
    float4 fragColor [[color(0)]];
};

struct fragment_function_in
{
    float2 texcoord [[user(locn0)]];
};

fragment fragment_function_out fragment_function(fragment_function_in in [[stage_in]], texture2d<float> tex [[texture(0)]], sampler texSmplr [[sampler(0)]])
{
    fragment_function_out out = {};
    out.fragColor = tex.sample(texSmplr, in.texcoord);
    return out;
}

