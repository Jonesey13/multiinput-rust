/*
Standard Polar Fragment Shader
 */

#version 330 core

in vec4 polar_geometry;
in vec4 color_geometry;
in vec2 emit_vertex;

uniform vec2 center;
uniform float aspect_ratio;

out vec4 color;

bool angleCompare(in float a, in vec2 range);

void main()
{
  vec2 fragCoord = emit_vertex;
  fragCoord.x = aspect_ratio * fragCoord.x;
  fragCoord -= center;
  float fragRadius = dot(fragCoord, fragCoord);
  bool radialOverlap = fragRadius >= dot(polar_geometry.x, polar_geometry.x)
    && fragRadius <= dot(polar_geometry.y, polar_geometry.y);

  bool angleOverlap = true;
  if(radialOverlap)
    {
      float angle = atan(fragCoord.y, fragCoord.x);
      angle = degrees(angle) / 360.0f;
      angleOverlap = angleCompare(angle, polar_geometry.zw);
    }
   if( angleOverlap && radialOverlap)
     color = color_geometry;
   else
     color = vec4(0.0f, 0.0f, 0.0f, 0.0f);
}

bool angleCompare(in float ang, in vec2 range)
{
   ang -=  floor(ang);
   range.x -= floor(range.x);
   range.y -= floor(range.x);

  bool isless = range.x <= range.y;
  if (isless)
    {
      return ang <=  range.y &&  ang >= range.x;
    }
  else
    {
      return ang >= range.x ||  ang <= range.y;
    }
}
