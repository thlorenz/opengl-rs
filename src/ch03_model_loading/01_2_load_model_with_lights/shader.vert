#version 330 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 texcoords;

out vec3 Normal;
out vec3 FragPos;
out vec2 TexCoords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
  vec4 world_pos = model * vec4(pos, 1.0);
  FragPos = vec3(world_pos);
  Normal = mat3(transpose(inverse(model))) * normal;
  TexCoords = texcoords;

  gl_Position = projection * view * world_pos;
}
