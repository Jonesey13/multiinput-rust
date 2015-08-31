/*
Tunnel Vertex Shader
 */

#version 330 core
in vec4 polar;
in vec4 color;

uniform float length_total;
uniform float length_circle;

out vec4 polar_vertex;
out vec4 color_vertex;

float render_polar(in float polar);

void main()
{
  float polar_x = render_polar(polar.x);
  float polar_y = render_polar(polar.y);
  polar_vertex = vec4(max(polar_x, 0.0f), max(polar_y, 0.0f), polar.zw);
  color_vertex = color;

  gl_Position = vec4(0.0f, 0.0f, 0.0f, 1.0f);
}

float render_polar(in float polar_in){
  float length_tunnel = length_total - length_circle;
  if (polar_in < length_circle){
    return polar_in / (length_circle*(length_tunnel + 1));
  }
  else{
    float polar_tunnel = max(length_tunnel - (polar_in - length_circle), -0.9);
    return 1.0 - (polar_tunnel / (polar_tunnel + 1));
  }
}
