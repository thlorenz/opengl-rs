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
    float theta = dot(lightDir, normalize(-light.direction));

    if (theta > light.cutOff)  {
      // Inside Spotlight
    
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
    } else {
      // Outside Spotlight use only ambient light
      FragColor = vec4(light.ambient * texture(material.diffuse, TexCoords).rgb, 1.0); 
    }





}
