#version 330 core

out vec4 FragColor;

in vec3 ourColor;
in vec2 textureCoords;

uniform sampler2D containerTexture;
uniform sampler2D smileyTexture;

void main() {
    FragColor = mix(
        texture(containerTexture, textureCoords),
        texture(smileyTexture, textureCoords),
        0.2
    );
}
