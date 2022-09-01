#version 100
attribute vec3 pos;
attribute vec4 color0;
attribute vec3 inst_pos;

varying lowp vec4 color;

uniform mat4 mvp;

void main() {
    vec4 pos = vec4(pos + inst_pos, 1.0);
    gl_Position = mvp * pos;
    color = color0;
}