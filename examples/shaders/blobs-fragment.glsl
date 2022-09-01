#version 100
precision highp float;

varying vec2 texcoord;

uniform float time;
uniform int blobs_count;
uniform vec2 blobs_positions[32];

float k = 20.0;
float field = 0.0;
vec2 coord;
    
void circle ( float r , vec3 col , vec2 offset) {
    vec2 pos = coord.xy;
    vec2 c = offset;
    float d = distance ( pos , c );
    field += ( k * r ) / ( d*d );
}
    
vec3 band ( float shade, float low, float high, vec3 col1, vec3 col2 ) {
    if ( (shade >= low) && (shade <= high) ) {
        float delta = (shade - low) / (high - low);
        vec3 colDiff = col2 - col1;
        return col1 + (delta * colDiff);
    }
    else
        return vec3(0.0,0.0,0.0);
}

vec3 gradient ( float shade ) {
    vec3 colour = vec3( (sin(time/2.0)*0.25)+0.25,0.0,(cos(time/2.0)*0.25)+0.25);
    
    vec3 col1 = vec3(0.01, 0.0, 1.0-0.01);
    vec3 col2 = vec3(1.0-0.01, 0.0, 0.01);
    vec3 col3 = vec3(0.02, 1.0-0.02, 0.02);
    vec3 col4 = vec3((0.01+0.02)/2.0, (0.01+0.02)/2.0, 1.0 - (0.01+0.02)/2.0);
    vec3 col5 = vec3(0.02, 0.02, 0.02);
    
    colour += band ( shade, 0.0, 0.3, colour, col1 );
    colour += band ( shade, 0.3, 0.6, col1, col2 );
    colour += band ( shade, 0.6, 0.8, col2, col3 );
    colour += band ( shade, 0.8, 0.9, col3, col4 );
    colour += band ( shade, 0.9, 1.0, col4, col5 );
    
    return colour;
}

void main() {
    coord = texcoord;
    
    for (int i = 0; i < 32; i++) {
        if (i >= blobs_count) { break; } // workaround for webgl error: Loop index cannot be compared with non-constant expression
        circle(.03 , vec3(0.7 ,0.2, 0.8), blobs_positions[i]);
    }
    
    float shade = min ( 1.0, max ( field/256.0, 0.0 ) );
    
    gl_FragColor = vec4( gradient(shade), 1.0 );
}