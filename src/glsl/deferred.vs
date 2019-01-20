#version 130
in vec4 position;
in vec2 uv;

smooth out vec2 fuv;

void main()
{
    gl_Position = position;
    fuv = uv;
}
