#version 330 core

struct Material {
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
  float shininess;
};

struct Light {
  vec3 position;
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};

out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;

uniform vec3 camera;
uniform Material material;
uniform Light light;

void main() {

    //
    // Ambient Light Calculation
    //
    vec3 ambient = light.ambient * material.ambient;

    //
    // Diffuse Light Calculation
    //
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPos);

    float normLightAngle = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * (normLightAngle * material.diffuse);

    //
    // Specular Light Calculation
    //
    vec3 cameraDir = normalize(camera - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float camLightAngle = dot(cameraDir, reflectDir);
    float spec = pow(max(camLightAngle, 0.0), material.shininess);
    vec3 specular = light.specular * (spec * material.specular);


    //
    // Final Color Calculation
    //
    vec3 result = ambient + diffuse + specular;

    FragColor = vec4(result, 1.0);
}