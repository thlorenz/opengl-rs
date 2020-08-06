#version 330 core

struct Material {
  sampler2D diffuse;
  sampler2D specular;
  sampler2D emission;
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
in vec2 TexCoords;

uniform vec3 camera;
uniform Material material;
uniform Light light;

void main() {

    //
    // Ambient Light Calculation
    //
    vec3 ambient = light.ambient * texture(material.diffuse, TexCoords).rgb;

    //
    // Diffuse Light Calculation
    //
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPos);

    float normLightAngle = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * normLightAngle * texture(material.diffuse, TexCoords).rgb;

    //
    // Specular Light Calculation
    //
    vec3 cameraDir = normalize(camera - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float camLightAngle = dot(cameraDir, reflectDir);
    float spec = pow(max(camLightAngle, 0.0), material.shininess);
    vec3 fullSpecular = texture(material.specular, TexCoords).rgb;
    vec3 specular = light.specular * spec * fullSpecular;

    //
    // Emission Light
    //
    vec3 fullEmission = texture(material.emission, TexCoords).rgb;
    // https://thebookofshaders.com/glossary/?search=step
    vec3 highlight = step(vec3(0.0), -fullSpecular);
    vec3 emission = fullEmission * highlight;

    //
    // Final Color Calculation
    //
    vec3 result = ambient + diffuse + specular + emission;

    FragColor = vec4(result, 1.0);
}
