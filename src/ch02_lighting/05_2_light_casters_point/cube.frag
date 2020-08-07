#version 330 core

struct Material {
  sampler2D diffuse;
  sampler2D specular;
  float shininess;
};

struct Light {
  vec3 position;

  vec3 ambient;
  vec3 diffuse;
  vec3 specular;

  float constant;
  float linear;
  float quadratic;
};

out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;
in vec2 TexCoords;

uniform vec3 camera;
uniform Material material;
uniform Light light;

void main() {
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPos);
    float dist = length(light.position - FragPos);
    float attentuation = 1.0 / (
        light.constant +
        light.linear * dist +
        light.quadratic * (dist * dist));

    //
    // Ambient Light Calculation
    //
    vec3 ambient = light.ambient * vec3(texture(material.diffuse, TexCoords)) * attentuation;

    //
    // Diffuse Light Calculation
    //

    float normLightAngle = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * normLightAngle * vec3(texture(material.diffuse, TexCoords)) * attentuation;

    //
    // Specular Light Calculation
    //
    vec3 cameraDir = normalize(camera - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float camLightAngle = dot(cameraDir, reflectDir);
    float spec = pow(max(camLightAngle, 0.0), material.shininess);
    vec3 specular = light.specular * spec * vec3(texture(material.specular, TexCoords)) * attentuation;


    //
    // Final Color Calculation
    //
    vec3 result = ambient + diffuse + specular;

    FragColor = vec4(result, 1.0);
}
