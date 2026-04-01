#version 330

// Inputs from Vertex Shader
in vec2 fragTexCoord;
in vec3 fragBrightness;

// Standard Uniforms
uniform sampler2D texture0; // Your Synty texture palette
uniform vec4 colDiffuse;    // Base color tint (usually WHITE)

out vec4 finalColor;

void main()
{
    // 1. Sample the texture.
    // Because the UVs are crushed to one pixel, this just pulls a solid color.
    vec4 texelColor = texture(texture0, fragTexCoord);

    // 2. Multiply the palette color by our RGB lighting and base tint
    vec3 rawColor = texelColor.rgb * colDiffuse.rgb * fragBrightness;

    // 3. Apply Gamma Correction to fix the dark .glb issue
    float gamma = 1.2;
    finalColor.rgb = pow(rawColor, vec3(1.0 / gamma));

    // 4. Pass the alpha straight through
    finalColor.a = texelColor.a * colDiffuse.a;
}
