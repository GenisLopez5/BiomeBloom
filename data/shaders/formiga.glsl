#version 330 core
out vec4 FragColor;

vec4 color = 
vec4(0.10, 0.09, 0.08, 1.0);
uniform float height;


void main() {
    FragColor = color;
    FragColor = mix(FragColor*0.8, FragColor, height);
}   

