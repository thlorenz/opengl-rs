#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 Normal;
out vec3 FragPos;

void main() {
  // fragment world coords
  FragPos = vec3(model * vec4(pos, 1.0));

  // cheap version which works fine as long as only uniform scaling occurs
  // Normal = normal;

  // alternative using normal 3x3 matrix created by transposing inverse model (inverse is expensive)
  Normal = mat3(transpose(inverse(model))) * normal;

  gl_Position = projection * view * model * vec4(pos, 1.0);
}
