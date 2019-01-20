#version 130

smooth in vec2 fuv; 

uniform float tick;
uniform sampler2D t_title; 
uniform sampler2D t_space; 

void main(void) {
    mat2 rot1 = mat2(cos(tick), -sin(tick), sin(tick), cos(tick));
    mat2 rot2 = mat2(cos(-tick/4.0), -sin(-tick/4.0), sin(-tick/4.0), cos(-tick/4.0));

    vec2 off1 = vec2(sin(tick), cos(tick));
    vec2 off2 = vec2(sin(-tick), cos(-tick));
    vec4 space = texture2D(t_space, ((fuv+off1)*rot1)+vec2(tick))/2.0 +
                 texture2D(t_space, ((fuv+off2)*rot2)+vec2(-tick/3.0));
    vec4 c = texture2D(t_title, fuv) + space;
    gl_FragColor = c;
}
