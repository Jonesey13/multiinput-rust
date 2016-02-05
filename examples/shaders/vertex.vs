#version 330
uniform mat4 wvp;

in vec3 position;
in vec3 normal;

out vec3 fnormal;
out vec3 vertex_position;


void main() {
  fnormal = normalize(normal);
  vertex_position = position;
  gl_Position = wvp * vec4(position, 1.0);
}
