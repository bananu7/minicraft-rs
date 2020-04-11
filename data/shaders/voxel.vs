#version 410 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 texCoord;
layout(location = 3) in vec3 color;

out VSOut {
    vec3 modelspacePosition;
    vec3 normal;
    vec2 texCoord;
    vec3 color;
} outData;

void main() {
    gl_Position = vec4(position, 1.0);
    outData.modelspacePosition = position.xyz;
    outData.normal = normal;
    outData.texCoord = texCoord;
    outData.color = color;
}