#version 330 core
layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 texCoords;

out vec2 TexCoords;

uniform bool leftMirror;

void main() {
    const float scale = 0.2;
    TexCoords = texCoords;
    // Place mirror to the top left or top right and scale the rendered image down
    float x = leftMirror 
        ? (pos.x * scale) - (1.0 - scale) 
        : (pos.x * scale) + (1.0 - scale);
    float y = (pos.y * scale) + (1.0 - scale);
    gl_Position = vec4(x, y, 0.0, 1.0);
}
