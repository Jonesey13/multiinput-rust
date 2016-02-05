#version 330

uniform vec3 tri_colour;
uniform vec3 light_source;

in vec3 fnormal;
in vec3 vertex_position;

out vec4 colour;

void main() {
  //  float diffuse = max(dot(vec3(0.0,0.0,-1.0), -normalize(light_source - vertex_position)),0.0);
  float diffuse = max(dot(fnormal, normalize(light_source - vertex_position)), 0.0);
  colour = vec4(diffuse * tri_colour, 1.0);
}
