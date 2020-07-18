 
#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

uniform mat4 projection_matrix;
uniform mat4 view_matrix;
uniform mat4 parent_model_matrix;
uniform mat4 model_matrix;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main()
{ 
    gl_Position = projection_matrix * view_matrix * parent_model_matrix * model_matrix * vec4(Position, 1.0);
    OUT.Color = Color;
}