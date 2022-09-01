#version 100
attribute vec4 pos;
attribute vec4 color0;

varying lowp vec4 color;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * pos;
    color = color0;
}