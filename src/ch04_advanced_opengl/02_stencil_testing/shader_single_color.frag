#version 330 core

out vec4 FragColor;

uniform sampler2D texture1;
uniform bool visualizeDepth;
  
void main() {             
    FragColor = vec4(0.8, 0.8, 0.1, 1.0);
}
