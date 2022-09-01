#version 100
varying lowp vec4 color;
varying lowp vec2 uv;

uniform sampler2D tex;

void main() {
    gl_FragColor = color * texture2D(tex, uv);
}