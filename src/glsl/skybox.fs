#version 130

smooth in vec2 fuv; 
uniform sampler2D t_day;
uniform sampler2D t_night; 

uniform vec3 eye; 
uniform vec3 sun;

void main(void) {
    vec4 c = texture2D(t_night, fuv); 
    gl_FragColor = c;
}
