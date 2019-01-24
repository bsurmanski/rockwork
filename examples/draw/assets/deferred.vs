in vec4 position;
in vec2 uv;

out vec2 uv_pos;

void main()
{
    gl_Position = position;
    uv_pos = uv;
}
