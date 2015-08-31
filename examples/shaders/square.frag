/*
Square Fragment Shader
 */

#version 330 core

in vec4 color_geometry;

out vec4 color;


void main()
{
  color = color_geometry;
}
