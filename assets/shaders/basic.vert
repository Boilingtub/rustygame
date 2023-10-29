#version 330 core

layout (location = 0) in vec3 aInd;
layout (location = 1) in vec3 aPos;
layout (location = 2) in vec4 aClr;

out VS_OUTPUT {
        vec4 clr;
} OUT;

void main()
{
        gl_Position = vec4(aPos, 1.0);
        OUT.clr = aClr;
}
