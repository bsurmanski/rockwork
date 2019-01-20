#version 130

uniform sampler2D t_color;
uniform sampler2D t_normal;
uniform sampler2D t_depth;
uniform sampler2D t_light;

smooth in vec2 fuv;

out vec4 outColor;

uniform float tick;
uniform float fade;

// gbuffer samples
vec4 color;
vec4 normal;
vec4 depth;
vec4 light;

vec4 applyLighting(vec4 c, vec4 l)
{
    return vec4((vec3(l) * vec3(c)) + vec3(l.a), 1.0);    
}

void main()
{
    float x = fuv.x * 2.0f - 1.0f;
    float xx = x*x;
    float xxxx = xx * xx;
    float texcox = -xxxx+1.0f;
    color = texture(t_color, vec2(fuv.x+ texcox * sin(tick * 5 + fuv.y*5)/100, fuv.y)); 
    normal = texture(t_normal, fuv);
    depth = texture(t_depth, fuv);
    light = texture(t_light, fuv);
    
    outColor = color + fade; //applyLighting(color, light);
}
