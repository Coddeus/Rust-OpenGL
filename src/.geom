#version 330 core
layout (triangles) in;
layout (triangle_strip, max_vertices = 101) out;

void main() {
    // Redrawing the base triangle
    gl_Position = gl_in[0].gl_Position;
    EmitVertex();
    gl_Position = gl_in[1].gl_Position;
    EmitVertex();
    gl_Position = gl_in[2].gl_Position;
    EmitVertex();

    // Drawing a Zig Zag from the triangle
    for (int i=3;i<100;i++) {
               if (i%4==3) {
            gl_Position = (1.0-1.0/float(i)) * gl_in[1].gl_Position + (1.0-1.0/float(i)) * gl_in[2].gl_Position;
            gl_Position.w = 1.0;
            EmitVertex();
        } else if (i%4==0) {
            gl_Position = (1.0-1.0/float(i)-1.0/float(i)) * gl_in[1].gl_Position + (1.0+1.0/float(i)-1.0/float(i)) * gl_in[2].gl_Position;
            gl_Position.w = 1.0;
            EmitVertex();
        } else if (i%4==1) {
            gl_Position = (1.0+1.0/float(i)-1.0/float(i)) * gl_in[1].gl_Position + (1.0-1.0/float(i)-1.0/float(i)) * gl_in[2].gl_Position;
            gl_Position.w = 1.0;
            EmitVertex();
        } else if (i%4==2) {
            gl_Position = (1.0-1.0/float(i)) * gl_in[1].gl_Position + (1.0-1.0/float(i)) * gl_in[2].gl_Position;
            gl_Position.w = 1.0;
            EmitVertex();
        }
    }

    EndPrimitive();
}