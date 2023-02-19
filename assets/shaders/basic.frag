#version 330 core
out vec4 Color;
in VS_OUTPUT {
        vec3 Pos;
} IN;

void main()
{
    Color = vec4(0.0f, IN.Pos.x , IN.Pos.y , 1.0f);
}
