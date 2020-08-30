#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D screenTexture;

vec4 kernel_effect(float[9] kernel) {
    const float offset = 1.0 / 300.0;
    vec2 offsets[9] = vec2[](
        vec2(-offset,  offset), // top-left
        vec2( 0.0f,    offset), // top-center
        vec2( offset,  offset), // top-right
        vec2(-offset,  0.0f),   // center-left
        vec2( 0.0f,    0.0f),   // center-center
        vec2( offset,  0.0f),   // center-right
        vec2(-offset, -offset), // bottom-left
        vec2( 0.0f,   -offset), // bottom-center
        vec2( offset, -offset)  // bottom-right    
    );

    vec3 sampleTex[9];
    for(int i = 0; i < 9; i++) {
        sampleTex[i] = vec3(texture(screenTexture, TexCoords.st + offsets[i]));
    }

    vec3 col = vec3(0.0);
    for(int i = 0; i < 9; i++) {
        col += sampleTex[i] * kernel[i];
    }
    
    return vec4(col, 1.0);
}

//
// Simple Effects
//

vec4 unprocessed(vec4 col) {
    return vec4(col.rgb, 1.0);
}

vec4 invert(vec4 col) {
    return vec4(vec3(1.0 - col.rgb), 1.0);
}

vec4 grayscale(vec4 col) {
    float average = 0.2126 * col.r + 0.7152 * col.g + 0.0722 * col.b;
    return vec4(average, average, average, 1.0);
}

//
// Kernel Effects
//

vec4 sharpen() {
    float kernel[9] = float[](
        -1, -1, -1,
        -1,  9, -1,
        -1, -1, -1
    );
    return kernel_effect(kernel);
}

vec4 blur() {
    float kernel[9] = float[](
        1.0 / 16, 2.0 / 16, 1.0 / 16,
        2.0 / 16, 4.0 / 16, 2.0 / 16,
        1.0 / 16, 2.0 / 16, 1.0 / 16  
    );
    return kernel_effect(kernel);
}

vec4 highlight_edges() {
    float kernel[9] = float[](
         1,  1,  1,
         1, -8,  1,
         1,  1,  1
    );
    return kernel_effect(kernel);
}

vec4 convolute() {
    float kernel[9] = float[](
         0, -1,  0,
        -1,  4, -1,
         0, -1,  0
    );
    return kernel_effect(kernel);
}

// More kernels and a nice explanation on how they work can be found at
// https://setosa.io/ev/image-kernels/

void main() {
    vec4 col = texture(screenTexture, TexCoords);
    // FragColor = unprocessed(col);
    // FragColor = invert(col);
    // FragColor = grayscale(col);
    FragColor = highlight_edges();
}
