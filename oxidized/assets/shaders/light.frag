#version 330

// Inputs from Vertex Shader
in vec2 fragTexCoord;
in vec4 fragColor;
in float fragBrightness;

// Standard Uniforms
uniform sampler2D texture0;
uniform vec4 colDiffuse;

// Output pixel
out vec4 finalColor;

void main()
{
    // 1. Get the texture color (defaults to white if none is assigned)
    vec4 texelColor = texture(texture0, fragTexCoord);

    // 2. Combine all color sources and apply lighting
    // Texture x Material Tint x Vertex Color x Brightness
    vec3 rawColor = texelColor.rgb * colDiffuse.rgb * fragColor.rgb * fragBrightness;

    // 3. Apply Gamma Correction (Linear -> sRGB for your monitor)
    float gamma = 1.2;
    finalColor.rgb = pow(rawColor, vec3(1.0 / gamma));

    // 4. Preserve the combined Alpha (opacity) channels
    finalColor.a = texelColor.a * colDiffuse.a * fragColor.a;
}
