#version 330 core

struct Material {
  sampler2D diffuse;
  sampler2D specular;
  float shininess;
};

struct Light {
  vec3 position;
  vec3 direction;

  vec3 ambient;
  vec3 diffuse;
  vec3 specular;

  float cutOff;
  float outerCutOff;
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
    vec3 lightDir = normalize(light.position - FragPos);

    // Ambient
    vec3 ambient = light.ambient * texture(material.diffuse, TexCoords).rgb;

    // Diffuse
    vec3 norm = normalize(Normal);
    float normLightAngle = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * normLightAngle * texture(material.diffuse, TexCoords).rgb;

    // Specular
    vec3 cameraDir = normalize(camera - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float camLightAngle = dot(cameraDir, reflectDir);
    float spec = pow(max(camLightAngle, 0.0), material.shininess);
    vec3 specular = light.specular * spec * texture(material.specular, TexCoords).rgb;

    // spotlight with soft edges
    float theta = dot(lightDir, normalize(-light.direction));
    float epsilon = light.cutOff - light.outerCutOff;
    float intensity = clamp((theta - light.outerCutOff) / epsilon, 0.0, 1.0);

    // Making light a bit stronger than in the tutorial + increasing reflection even more
    // which improves the flash light effect.
    diffuse *= (intensity * 1.5);
    specular *= (intensity * 2.0);

    // Attenuation
    float dist = length(light.position - FragPos);
    float attenuation = 1.0 / (
        light.constant +
        light.linear * dist +
        light.quadratic * (dist * dist));

    diffuse *= attenuation;
    specular *= attenuation;

    vec3 result = ambient + diffuse + specular;
    FragColor = vec4(result, 1.0);
}
