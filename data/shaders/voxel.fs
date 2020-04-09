#version 410 core

uniform sampler2D colorMap;
uniform sampler2D normalMap;
uniform sampler2D depthMap;

in TESOut {
    vec3 color;
    vec2 texCoord;
} vertex;

out vec4 outColor;

void main() {
    //outColor = vec4(vertex.color, 1.0);
    outColor = vec4(1.0, 0.0, 0.0, 1.0);
    outColor = vec4(vertex.texCoord, 0.0, 1.0);
}
