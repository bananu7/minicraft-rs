#version 410 core

in vec3 vColor;
out vec4 f_color;

void main() {
    f_color = vec4(vColor, 1.0);
}