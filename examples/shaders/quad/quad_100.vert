#version 100

struct Uniforms
{
    vec2 offset;
};

uniform Uniforms _22;

attribute vec2 pos;
varying vec2 texcoord;
attribute vec2 uv;

void main()
{
    gl_Position = vec4(pos + _22.offset, 0.0, 1.0);
    texcoord = uv;
}

