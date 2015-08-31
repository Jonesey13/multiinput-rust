/*
Square Geometry Shader
 */

#version 330 core

layout(points) in;
layout(triangle_strip, max_vertices = 4) out;

in vec4 square_spec[];
in vec4 color_vertex[];
out vec4 color_geometry;

uniform float aspect_ratio;

void main() {

  color_geometry = color_vertex[0];

  vec2 pos = square_spec[0].xy;
  float half_width = square_spec[0].z;

  gl_Position = vec4(pos.x - half_width, pos.y - half_width, 0.0f, 1.0f);
  gl_Position.x = gl_Position.x / aspect_ratio;
  EmitVertex();
  gl_Position = vec4(pos.x - half_width, pos.y + half_width, 0.0f, 1.0f);
  gl_Position.x = gl_Position.x / aspect_ratio;
  EmitVertex();
  gl_Position = vec4(pos.x + half_width, pos.y - half_width, 0.0f, 1.0f);
  gl_Position.x = gl_Position.x / aspect_ratio;
  EmitVertex();
  gl_Position = vec4(pos.x + half_width, pos.y + half_width, 0.0f, 1.0f);
  gl_Position.x = gl_Position.x / aspect_ratio;
  EmitVertex();

  EndPrimitive();
}
