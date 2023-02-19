#version 330 core

layout (location = 0) in vec3 aPos;

out VS_OUTPUT {
        vec3 Pos;
} OUT;

void main()
{
        gl_Position = vec4(aPos, 1.0);
        OUT.Pos = aPos;
}
