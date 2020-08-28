#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;

struct Uniforms
{
    float2 offset;
};

struct vertex_function_out
{
    float2 texcoord [[user(locn0)]];
    float4 gl_Position [[position]];
};

struct vertex_function_in
{
    float2 pos [[attribute(0)]];
    float2 uv [[attribute(1)]];
};

vertex vertex_function_out vertex_function(vertex_function_in in [[stage_in]], constant Uniforms& _22 [[buffer(0)]])
{
    vertex_function_out out = {};
    out.gl_Position = float4(in.pos + _22.offset, 0.0, 1.0);
    out.texcoord = in.uv;
    return out;
}

