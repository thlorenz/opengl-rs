#version 330 core

out vec4 FragColor;
in vec2 TexCoords;

float near = 0.1; 
float far  = 100.0; 

in vec3 Normal;
in vec3 FragPos;

uniform vec3 camera;
uniform samplerCube skybox;

void main() {             
    vec3 I = normalize(FragPos - camera);
    vec3 R = reflect(I, normalize(Normal));
    FragColor = vec4(texture(skybox, R).rgb, 1.0);
}
