#version 330 core

out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 lightPos;

float ambientStrength = 0.1;

void main() {
    vec3 norm = normalize(Normal);

    // direction vector between light source and fragment's position
    vec3 lightDir = normalize(lightPos - FragPos);

    // diffuse impact the light has on current fragment
    // diff is the angle between the light and the fragment normal
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    vec3 ambient = ambientStrength * lightColor;
    vec3 result = (ambient + diffuse) * objectColor;
    FragColor = vec4(result, 1.0);
}
