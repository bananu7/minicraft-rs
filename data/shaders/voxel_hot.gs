#version 410 core

layout (points) in;
layout (triangle_strip, max_vertices = 24) out;

uniform vec3 chunk_position;

in vec3 vs_position[];
in int vs_value[];

out vec3 out_position;
out vec3 out_normal;
out vec2 out_texCoord;
out vec3 out_color;

void main() {
    if (vs_value[0] == 0)
        return;

    vec3 world_pos = chunk_position + vs_position[0];

    // top
    out_position = world_pos + vec3(0.0, 1.0, 0.0);
    out_normal = vec3(0.0, 1.0, 0.0);
    out_texCoord = vec2(0.0, 0.0);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(0.0, 1.0, 1.0);
    out_normal = vec3(0.0, 1.0, 0.0);
    out_texCoord = vec2(0.0, 1.0);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 1.0, 0.0);
    out_normal = vec3(0.0, 1.0, 0.0);
    out_texCoord = vec2(1.0, 0.0);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 1.0, 1.0);
    out_normal = vec3(0.0, 1.0, 0.0);
    out_texCoord = vec2(1.0, 1.0);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    EndPrimitive();

    // bottom
    out_position = world_pos + vec3(0.0, 0.0, 0.0);
    out_normal = vec3(0.0, -1.0, 0.0);
    out_texCoord = vec2(0.0, 0.0);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(0.0, 0.0, 1.0);
    out_normal = vec3(0.0, -1.0, 0.0);
    out_texCoord = vec2(0.0, 1.0);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 0.0, 0.0);
    out_normal = vec3(0.0, -1.0, 0.0);
    out_texCoord = vec2(1.0, 0.0);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 0.0, 1.0);
    out_normal = vec3(0.0, -1.0, 0.0);
    out_texCoord = vec2(1.0, 1.0);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    EndPrimitive();

    // back
    out_position = world_pos + vec3(0.0, 0.0, 1.0);
    out_normal = vec3(0.0, 0.0, 1.0);
    out_texCoord = vec2(0.0, 0.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(0.0, 1.0, 1.0);
    out_normal = vec3(0.0, 0.0, 1.0);
    out_texCoord = vec2(0.0, 1.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 0.0, 1.0);
    out_normal = vec3(0.0, 0.0, 1.0);
    out_texCoord = vec2(1.0, 0.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 1.0, 1.0);
    out_normal = vec3(0.0, 0.0, 1.0);
    out_texCoord = vec2(1.0, 1.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    EndPrimitive();

    // front
    out_position = world_pos + vec3(0.0, 0.0, 0.0);
    out_normal = vec3(0.0, 0.0, -1.0);
    out_texCoord = vec2(0.0, 0.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(0.0, 1.0, 0.0);
    out_normal = vec3(0.0, 0.0, -1.0);
    out_texCoord = vec2(0.0, 1.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 0.0, 0.0);
    out_normal = vec3(0.0, 0.0, -1.0);
    out_texCoord = vec2(1.0, 0.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 1.0, 0.0);
    out_normal = vec3(0.0, 0.0, -1.0);
    out_texCoord = vec2(1.0, 1.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    EndPrimitive();

    // right
    out_position = world_pos + vec3(1.0, 0.0, 0.0);
    out_normal = vec3(1.0, 0.0, 0.0);
    out_texCoord = vec2(0.0, 0.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 0.0, 1.0);
    out_normal = vec3(1.0, 0.0, 0.0);
    out_texCoord = vec2(0.0, 1.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 1.0, 0.0);
    out_normal = vec3(1.0, 0.0, 0.0);
    out_texCoord = vec2(1.0, 0.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 1.0, 1.0);
    out_normal = vec3(1.0, 0.0, 0.0);
    out_texCoord = vec2(1.0, 1.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    EndPrimitive();

    // left
    out_position = world_pos + vec3(0.0, 0.0, 0.0);
    out_normal = vec3(-1.0, 0.0, 0.0);
    out_texCoord = vec2(0.0, 0.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(0.0, 0.0, 1.0);
    out_normal = vec3(-1.0, 0.0, 0.0);
    out_texCoord = vec2(0.0, 1.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(0.0, 1.0, 0.0);
    out_normal = vec3(-1.0, 0.0, 0.0);
    out_texCoord = vec2(1.0, 0.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(0.0, 1.0, 1.0);
    out_normal = vec3(-1.0, 0.0, 0.0);
    out_texCoord = vec2(1.0, 1.0);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    EndPrimitive();
}  