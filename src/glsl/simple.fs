#version 130

smooth in vec2 texco;
uniform sampler2D tex;

void main(void) {
    vec4 c = texture2D(tex, texco);
    if(c.a < 0.1) discard;
    gl_FragColor = c;
}
