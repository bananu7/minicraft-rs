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
    int value = vs_value[0];
    if (value == 0)
        return;

    // 2by2
    const int tex_in_atlas = 2;
    const float tc_width = 1.0 / float(tex_in_atlas);
    const float tc_height = 1.0 / float(tex_in_atlas);

    float tc_left = float((value-1) % tex_in_atlas) * tc_width;
    float tc_bottom = float((value-1) / tex_in_atlas) * tc_width;
    float tc_right = tc_left + tc_width;
    float tc_top = tc_bottom + tc_height;

    vec3 world_pos = chunk_position + vs_position[0];

    // top
    out_position = world_pos + vec3(0.0, 1.0, 0.0);
    out_normal = vec3(0.0, 1.0, 0.0);
    out_texCoord = vec2(tc_left, tc_bottom);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(0.0, 1.0, 1.0);
    out_normal = vec3(0.0, 1.0, 0.0);
    out_texCoord = vec2(tc_left, tc_top);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 1.0, 0.0);
    out_normal = vec3(0.0, 1.0, 0.0);
    out_texCoord = vec2(tc_right, tc_bottom);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 1.0, 1.0);
    out_normal = vec3(0.0, 1.0, 0.0);
    out_texCoord = vec2(tc_right, tc_top);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    EndPrimitive();

    // bottom
    out_position = world_pos + vec3(0.0, 0.0, 0.0);
    out_normal = vec3(0.0, -1.0, 0.0);
    out_texCoord = vec2(tc_left, tc_bottom);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(0.0, 0.0, 1.0);
    out_normal = vec3(0.0, -1.0, 0.0);
    out_texCoord = vec2(tc_left, tc_top);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 0.0, 0.0);
    out_normal = vec3(0.0, -1.0, 0.0);
    out_texCoord = vec2(tc_right, tc_bottom);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 0.0, 1.0);
    out_normal = vec3(0.0, -1.0, 0.0);
    out_texCoord = vec2(tc_right, tc_top);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();

    EndPrimitive();

    // back
    out_position = world_pos + vec3(0.0, 0.0, 1.0);
    out_normal = vec3(0.0, 0.0, 1.0);
    out_texCoord = vec2(tc_left, tc_bottom);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(0.0, 1.0, 1.0);
    out_normal = vec3(0.0, 0.0, 1.0);
    out_texCoord = vec2(tc_left, tc_top);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 0.0, 1.0);
    out_normal = vec3(0.0, 0.0, 1.0);
    out_texCoord = vec2(tc_right, tc_bottom);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 1.0, 1.0);
    out_normal = vec3(0.0, 0.0, 1.0);
    out_texCoord = vec2(tc_right, tc_top);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    EndPrimitive();

    // front
    out_position = world_pos + vec3(0.0, 0.0, 0.0);
    out_normal = vec3(0.0, 0.0, -1.0);
    out_texCoord = vec2(tc_left, tc_bottom);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(0.0, 1.0, 0.0);
    out_normal = vec3(0.0, 0.0, -1.0);
    out_texCoord = vec2(tc_left, tc_top);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 0.0, 0.0);
    out_normal = vec3(0.0, 0.0, -1.0);
    out_texCoord = vec2(tc_right, tc_bottom);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 1.0, 0.0);
    out_normal = vec3(0.0, 0.0, -1.0);
    out_texCoord = vec2(tc_right, tc_top);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    EndPrimitive();

    // right
    out_position = world_pos + vec3(1.0, 0.0, 0.0);
    out_normal = vec3(1.0, 0.0, 0.0);
    out_texCoord = vec2(tc_left, tc_bottom);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 0.0, 1.0);
    out_normal = vec3(1.0, 0.0, 0.0);
    out_texCoord = vec2(tc_left, tc_top);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 1.0, 0.0);
    out_normal = vec3(1.0, 0.0, 0.0);
    out_texCoord = vec2(tc_right, tc_bottom);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(1.0, 1.0, 1.0);
    out_normal = vec3(1.0, 0.0, 0.0);
    out_texCoord = vec2(tc_right, tc_top);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    EndPrimitive();

    // left
    out_position = world_pos + vec3(0.0, 0.0, 0.0);
    out_normal = vec3(-1.0, 0.0, 0.0);
    out_texCoord = vec2(tc_left, tc_bottom);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(0.0, 0.0, 1.0);
    out_normal = vec3(-1.0, 0.0, 0.0);
    out_texCoord = vec2(tc_left, tc_top);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(0.0, 1.0, 0.0);
    out_normal = vec3(-1.0, 0.0, 0.0);
    out_texCoord = vec2(tc_right, tc_bottom);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    out_position = world_pos + vec3(0.0, 1.0, 1.0);
    out_normal = vec3(-1.0, 0.0, 0.0);
    out_texCoord = vec2(tc_right, tc_top);
    out_color = vec3(1.0, 1.0, 1.0);
    EmitVertex();

    EndPrimitive();
}  