#version 330
#define     MAX_LIGHTS              64

// Inputs from Vertex Shader
in vec2 fragTexCoord;
in vec4 fragColor;
in float fragBrightness;

in vec3 fragPosition;
in vec3 fragNormal;

// Standard Uniforms
uniform sampler2D texture0; // Albedo
uniform sampler2D emission; // Emission
uniform vec4 colDiffuse;

struct Light {
    int enabled;
    float falloff;
    vec3 position;
    vec3 direction;
    vec4 color;
};

uniform Light lights[MAX_LIGHTS];
uniform int numLights;

// Output pixel
out vec4 finalColor;

void main()
{
    // 1. Get the texture color (defaults to white if none is assigned)
    vec4 texelColor = texture(texture0, fragTexCoord);
    vec4 emissionColor = texture(emission, fragTexCoord);
    vec3 normal = normalize(fragNormal);
    vec3 totalLighting = vec3(0.0);

    for (int i = 0; i < numLights; i++)
    {
        if (lights[i].enabled == 1)
        {
            // Vector FROM the pixel TO the light (used for diffuse lighting)
            vec3 pixelToLight = lights[i].position - fragPosition;
            float distance = length(pixelToLight);

            if (distance > lights[i].falloff) continue;

            vec3 lightDir = normalize(pixelToLight);
            //
            // // NEW: Vector FROM the light TO the pixel (used for direction check)
            vec3 lightToPixel = -lightDir;
            //
            // // NEW: Check if pixel is in front of or behind the light's facing direction
            // // dot() returns > 0.0 if in front, < 0.0 if behind
            float frontBackCheck = dot(normalize(lights[i].direction), lightToPixel);

            // NEW: If the pixel is behind the light, artificially inflate the distance
            // so the light falls off MUCH faster (e.g., 10x faster)
            float distancePenalty = 1.0;
            if (frontBackCheck < 0.0) {
                distancePenalty = 10.0; // Shrinks the effective radius behind the light to 10%
            }

            float modifiedDistance = distance * distancePenalty;

            // Calculate Attenuation with the modified distance
            float attenuation = clamp(1.0 - (modifiedDistance * modifiedDistance) / (lights[i].falloff * lights[i].falloff), 0.0, 1.0);
            attenuation *= attenuation;

            // If the modified distance pushed the attenuation to 0, skip the lighting math
            if (attenuation <= 0.0) continue;

            // Calculate Diffuse
            float diff = max(dot(normal, lightDir), 0.0);

            // Combine
            vec3 diffuseLight = lights[i].color.rgb * diff * attenuation;
            totalLighting += diffuseLight;
        }
    }


    // 2. Combine all color sources and apply lighting
    // Texture x Material Tint x Vertex Color x Brightness
    vec3 rawColor = texelColor.rgb * colDiffuse.rgb * fragColor.rgb * clamp(fragBrightness/2.0 + totalLighting, 0.0, 1.0);
    rawColor = rawColor + emissionColor.rgb;


    // 3. Apply Gamma Correction (Linear -> sRGB for your monitor)
    float gamma = 1.2;
    finalColor.rgb = pow(rawColor, vec3(1.0 / gamma));

    // 4. Preserve the combined Alpha (opacity) channels
    finalColor.a = texelColor.a * colDiffuse.a * fragColor.a;
}
