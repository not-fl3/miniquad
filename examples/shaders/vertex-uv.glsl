#version 100
attribute vec2 pos;
attribute vec2 uv;

uniform vec2 offset;

varying highp vec2 texcoord;

void main() {
    gl_Position = vec4(pos + offset, 0, 1);
    texcoord = uv;
}