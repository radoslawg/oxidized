#version 330

in vec2 fragTexCoord;
in vec3 fragBrightness; // NEW: Taking in the 3-part brightness

uniform vec4 colDiffuse;

out vec4 finalColor;

void main()
{
    // Apply the separate RGB brightness values to the base material color
    vec3 rawColor = colDiffuse.rgb * fragBrightness;

    // Apply our Gamma Correction so it isn't too dark
    float gamma = 2.2;
    finalColor.rgb = pow(rawColor, vec3(1.0 / gamma));

    // Keep the original opacity
    finalColor.a = colDiffuse.a;
}
