#version 130

in vec3 position;
in vec3 normal;
in vec2 uv;

smooth out vec2 fuv;

void main(void) {
    gl_Position = vec4(position, 1.0f);
    fuv = uv;
}
