#version 410 core

layout(location = 0) in vec3 position;
layout(location = 1) in int value;
out vec3 vs_position;
out int vs_value;

void main() {
    gl_Position = vec4(position, 1.0);
    vs_position = position;
    vs_value = value;
}