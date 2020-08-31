#version 330 core

out vec4 FragColor;
in vec2 TexCoords;

float near = 0.1; 
float far  = 100.0; 

in vec3 Normal;
in vec3 FragPos;

uniform vec3 camera;
uniform samplerCube skybox;

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
      vec3 I = normalize(FragPos - camera);
      vec3 R = reflect(I, normalize(Normal));
      FragColor = vec4(texture(skybox, R).rgb, 1.0);
  }
}
