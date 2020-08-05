#version 330 core

out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;
in vec3 LightPos;

uniform vec3 objectColor;
uniform vec3 lightColor;

float ambientStrength = 0.1;
float specularStrength = 0.5;
float shininess = 32.0;

void main() {
    vec3 norm = normalize(Normal);

    // direction between light source and fragment's position
    vec3 lightDir = normalize(LightPos - FragPos);

    //
    // Ambient Light Calculation
    //
    vec3 ambient = ambientStrength * lightColor;

    //
    // Diffuse Light Calculation
    //

    // angle between the light and the fragment normal
    float normLightAngle = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = normLightAngle * lightColor;

    //
    // Specular Light Calculation
    //

    // direction between camera (viewer) and fragment's position
    // when doing this calculation in view-space, the camera (view)
    // is always at (0, 0, 0)
    vec3 cameraDir = normalize(-FragPos);
    // direction of light reflecting off fragment
    vec3 reflectDir = reflect(-lightDir, norm);

    // angle between camera direction and reflected light
    float camLightAngle = dot(cameraDir, reflectDir);
    float spec = pow(max(camLightAngle, 0.0), shininess);
    vec3 specular = specularStrength * spec * lightColor;


    //
    // Final Color Calculation
    //
    vec3 result = (ambient + diffuse + specular) * objectColor;

    FragColor = vec4(result, 1.0);
}
