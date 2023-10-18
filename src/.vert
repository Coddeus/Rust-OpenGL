#version 330 core

layout (location = 0) in vec2 Position;

uniform float u_time;

mat2 rotate(float angle)
{
    float c = cos(angle);
    float s = sin(angle);

    return mat2(c, -s, s, c);
}

void main()
{
    
    gl_Position = vec4(Position * rotate(u_time), 0.0, 1.0);
}