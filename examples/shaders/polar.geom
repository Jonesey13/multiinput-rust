/*
Standard Polar Geometry Shader
 */

#version 330 core

layout(points) in;
layout(triangle_strip, max_vertices = 4) out;

in vec4 polar_vertex[];
in vec4 color_vertex[];
out vec4 color_geometry;
out vec4 polar_geometry;
out vec2 emit_vertex; // For Giving the Screen Position to the Fragments

uniform float aspect_ratio;

void main() {

  color_geometry = color_vertex[0];
  polar_geometry = polar_vertex[0];

  if (polar_vertex[0].w - polar_vertex[0].z <= 0.25)
    {
      float angle_first = radians(polar_vertex[0].z * 360);
      float angle_second = radians(polar_vertex[0].w * 360);
      float angle_diff = (angle_first - angle_second) / 2.0f;
      float radial_large = polar_vertex[0].y / cos(angle_diff);
      gl_Position = vec4(radial_large * vec2(cos(angle_first), sin(angle_first)), 0.0f, 1.0f);
      gl_Position.x = gl_Position.x / aspect_ratio;
      emit_vertex = gl_Position.xy;
      EmitVertex();
      gl_Position = vec4(polar_vertex[0].x * vec2(cos(angle_first), sin(angle_first)), 0.0f, 1.0f);
      gl_Position.x = gl_Position.x / aspect_ratio;
      emit_vertex = gl_Position.xy;
      EmitVertex();
      gl_Position = vec4(radial_large * vec2(cos(angle_second), sin(angle_second)), 0.0f, 1.0f);
      gl_Position.x = gl_Position.x / aspect_ratio;
      emit_vertex = gl_Position.xy;
      EmitVertex();
      gl_Position = vec4(polar_vertex[0].x * vec2(cos(angle_second), sin(angle_second)), 0.0f, 1.0f);
      gl_Position.x = gl_Position.x / aspect_ratio;
      emit_vertex = gl_Position.xy;
      EmitVertex();
    }
  else
    {
      gl_Position = vec4(polar_vertex[0].y * vec2(-1.0f, -1.0f), 0.0f, 1.0f);
      gl_Position.x = gl_Position.x / aspect_ratio;
      emit_vertex = gl_Position.xy;
      EmitVertex();
      gl_Position = vec4(polar_vertex[0].y * vec2(-1.0f, 1.0f), 0.0f, 1.0f);
      gl_Position.x = gl_Position.x / aspect_ratio;
      emit_vertex = gl_Position.xy;
      EmitVertex();
      gl_Position = vec4(polar_vertex[0].y * vec2(1.0f,- 1.0f), 0.0f, 1.0f);
      gl_Position.x = gl_Position.x / aspect_ratio;
      emit_vertex = gl_Position.xy;
      EmitVertex();
      gl_Position = vec4(polar_vertex[0].y * vec2(1.0f, 1.0f), 0.0f, 1.0f);
      gl_Position.x = gl_Position.x / aspect_ratio;
      emit_vertex = gl_Position.xy;
      EmitVertex();
    }

    EndPrimitive();
}
