#version 330 core

out vec4 FragColor;
in vec2 TexCoords;

float near = 0.1; 
float far  = 100.0; 

uniform sampler2D texture_diffuse1;
uniform bool visualizeDepth;
  
float LinearizeDepth(float depth) {
    float z = depth * 2.0 - 1.0; // back to NDC 
    return (2.0 * near * far) / (far + near - z * (far - near));	
}

void main() {             
  if (visualizeDepth) {
    float depth = LinearizeDepth(gl_FragCoord.z) / far;
    FragColor = vec4(vec3(depth), 1.0);
  } else {
    vec4 color = texture(texture_diffuse1, TexCoords);
    if (color.a < 0.5) {
        discard;
    }
    FragColor = color;
  }
}
