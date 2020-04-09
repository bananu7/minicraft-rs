#version 440 core

layout (points) in;
layout (triangle_strip, max_vertices = 4) out;
// size of one vertex - 44
// size of one cube - 6*44 =  264

/*layout(xfb_buffer = 0, xfb_stride = 44) out Data {
    vec3 position;
    vec3 color;
    vec3 normal;
    vec2 texCoord;
};*/

in vec3 vs_position[];
in int vs_value[];
/*layout(xfb_buffer = 0)*/ out vec3 out_position;
/*layout(xfb_buffer = 0)*/ out vec3 out_normal;
/*layout(xfb_buffer = 0)*/ out vec2 out_texCoord;
/*layout(xfb_buffer = 0)*/ out vec3 out_color;

void main() {
    if (vs_value[0] == 0)
        return;

    //gl_Position = gl_in[0].gl_Position + vec4(0.0, 1.0, 0.0, 0.0);
    out_position = vs_position[0] + vec3(0.0, 1.0, 0.0);
    out_normal = vec3(0.0, 1.0, 0.0);
    out_texCoord = vec2(0.0, 0.0);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();
    //gl_Position = gl_in[0].gl_Position + vec4(0.0, 1.0, 1.0, 0.0);
    out_position = vs_position[0] + vec3(0.0, 1.0, 1.0);
    out_normal = vec3(0.0, 1.0, 0.0);
    out_texCoord = vec2(0.0, 0.0);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();
    //gl_Position = gl_in[0].gl_Position + vec4(1.0, 1.0, 0.0, 0.0);
    out_position = vs_position[0] + vec3(1.0, 1.0, 0.0);
    out_normal = vec3(0.0, 1.0, 0.0);
    out_texCoord = vec2(0.0, 0.0);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();
    //gl_Position = gl_in[0].gl_Position + vec4(1.0, 1.0, 1.0, 0.0);
    out_position = vs_position[0] + vec3(1.0, 1.0, 1.0);
    out_normal = vec3(0.0, 1.0, 0.0);
    out_texCoord = vec2(0.0, 0.0);
    out_color = vec3(1.0, 0.0, 1.0);
    EmitVertex();                

    EndPrimitive();
}  