/*
Square Vertex Shader
 */

#version 330 core
in vec3 square;
in vec4 color;

out vec3 square_spec;
out vec4 color_vertex;

void main()
{
  square_spec = square;
  color_vertex = color;

  gl_Position = vec4(0.0f, 0.0f, 0.0f, 1.0f);
}
