#version 330 core

// in VS_OUTPUT {
//     vec2 Position;
// } IN;

out vec4 Color;

// uniform float u_time;

void main()
{
    // vec2 uv = vec2(u_resolution.x*IN.Position.x, u_resolution.y*IN.Position.y); 
    // vec4 Color = vec4(IN.Color*min(1.0-length(IN.Position), 1.0), 1.0);
    Color = vec4(1.0, 0.902, 0.0, 1.0);
    if (length(gl_FragCoord-500.0) < 745.0 && length(gl_FragCoord-500.0) > 743.0) {
        Color = vec4(54./255., 159./255., 219./255., 1.0);
    }
}