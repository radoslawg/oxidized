#version 100

precision mediump float;

// Inputs from Vertex Shader
varying vec2 fragTexCoord;
varying vec4 fragColor;
varying float fragBrightness;

// Standard Uniforms
uniform sampler2D texture0;
uniform vec4 colDiffuse;

void main()
{
    // 1. Get the texture color (defaults to white if none is assigned)
    vec4 texelColor = texture2D(texture0, fragTexCoord);

    // 2. Combine all color sources and apply lighting
    // Texture x Material Tint x Vertex Color x Brightness
    vec3 rawColor = texelColor.rgb * colDiffuse.rgb * fragColor.rgb * fragBrightness;

    // 3. Apply Gamma Correction (Linear -> sRGB for your monitor)
    float gamma = 1.2;
    gl_FragColor.rgb = pow(rawColor, vec3(1.0 / gamma));

    // 4. Preserve the combined Alpha (opacity) channels
    gl_FragColor.a = texelColor.a * colDiffuse.a * fragColor.a;
}
