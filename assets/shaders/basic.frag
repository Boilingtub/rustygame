#version 330 core
out vec4 Color;
in VS_OUTPUT {
        vec4 clr;
} IN;

void main()
{
    Color = vec4(0.0,0.0,1.0,1.0);
}
