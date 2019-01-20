#version 130

in vec3 position;
in vec3 normal;
in vec2 uv;

uniform mat4 v_matrix;
uniform mat4 p_matrix;

smooth out vec3 eye;
smooth out vec2 fuv;

void main(void) {
    eye = (v_matrix * vec4(position, 1.0f)).xyz;
    gl_Position = p_matrix * v_matrix * vec4(position, 1.0f);
    fuv = uv;
}
