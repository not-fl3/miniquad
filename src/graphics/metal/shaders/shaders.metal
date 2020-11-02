#include <metal_stdlib>
using namespace metal;

struct ColorInOut {
    float4 position [[position]];
    float4 color;
};

struct Rect {
    float x;
    float y;
    float w;
    float h;
};

struct Color {
    float r;
    float g;
    float b;
    float a;
};

struct ClearRect {
    Rect rect;
    Color color;
};

float2 rect_vert(
    Rect rect,
    uint vid
) {
    float2 pos;

    float left = rect.x;
    float right = rect.x + rect.w;
    float bottom = rect.y;
    float top = rect.y + rect.h;

    switch (vid) {
    case 0:
        pos = float2(right, top);
        break;
    case 1:
        pos = float2(left, top);
        break;
    case 2:
        pos = float2(right, bottom);
        break;
    case 3:
        pos = float2(left, bottom);
        break;
    }
    return pos;
}

vertex ColorInOut clear_rect_vertex(
    const device ClearRect *clear_rect [[ buffer(0) ]],
    unsigned int vid [[ vertex_id ]]
) {
    ColorInOut out;
    float4 pos = float4(rect_vert(clear_rect->rect, vid), 0, 1);
    auto col = clear_rect->color;

    out.position = pos;
    out.color = float4(col.r, col.g, col.b, col.a);
    return out;
}

fragment float4 clear_rect_fragment(ColorInOut in [[stage_in]])
{
    return in.color;
};
