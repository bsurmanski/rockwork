in vec2 uv_pos;

out vec4 color;

void main()
{
    color = vec4(uv_pos, 1.0, 1.0);
}
