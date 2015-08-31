/*
Standard Polar Vertex Shader
 */

#version 330 core
in vec4 polar;
in vec4 color;

uniform float radial_shift;

out vec4 polar_vertex;
out vec4 color_vertex;

void main()
{
  polar_vertex = vec4(max(polar.x - radial_shift, 0.0f), max(polar.y - radial_shift, 0.0f), polar.zw);
  color_vertex = color;

  gl_Position = vec4(0.0f, 0.0f, 0.0f, 1.0f);
}
