#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 lightPos;

out vec3 Normal;
out vec3 FragPos;
out vec3 LightPos;

void main() {
  // transform world-space lightPos into view-space lightPos
  LightPos = vec3(view * vec4(lightPos, 1.0));

  // fragment view coords
  FragPos = vec3(view * model * vec4(pos, 1.0));

  // cheap version which works fine as long as only uniform scaling occurs
  // Normal = normal;

  // alternative using normal 3x3 matrix created by transposing inverse model (inverse is expensive)
  // note that this is now performed in view-space
  Normal = mat3(transpose(inverse(view * model))) * normal;

  gl_Position = projection * view * model * vec4(pos, 1.0);
}
