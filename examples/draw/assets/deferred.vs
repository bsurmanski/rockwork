in vec4 position;
in vec2 uv;

out vec2 fuv;

void main()
{
    gl_Position = position;
    fuv = uv;
}
