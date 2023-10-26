#version 330 core

uniform float u_time;

out vec4 Color;

void main()
{
    float sun_lifetime = mod(u_time, 6.0);
    Color = vec4(
        sun_lifetime / 6.0 * 0.8 + 0.2,
        0.2 + 0.4 * min(sun_lifetime, 3.0) / 3.0 - (0.6 * (max(sun_lifetime, 3.0) / 3.0 - 1.0)),
        1.0-sun_lifetime / 6.0,
        1.0
    );
}