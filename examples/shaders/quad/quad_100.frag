#version 100
precision mediump float;
precision highp int;

uniform highp sampler2D tex;

varying highp vec2 texcoord;

void main()
{
    gl_FragData[0] = texture2D(tex, texcoord);
}

