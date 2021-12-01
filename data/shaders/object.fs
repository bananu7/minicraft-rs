#version 410 core

uniform vec3 eye;
//uniform float time;

in VSOut {
    vec3 modelspacePosition;
    //vec3 normal;
    //vec2 texCoord;
    //vec3 color;
} vertex;

out vec4 outColor;

void main() {
    vec3 color = vec3(1.0, 0.0, 0.0);
    outColor = vec4(color, 1.0);
}
