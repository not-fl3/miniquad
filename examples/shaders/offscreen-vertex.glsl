#version 100
attribute vec4 pos;
attribute vec4 color0;
attribute vec2 uv0;

varying lowp vec4 color;
varying lowp vec2 uv;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * pos;
    color = color0;
    uv = uv0;
}