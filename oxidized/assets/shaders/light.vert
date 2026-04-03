#version 330
#define     MAX_LIGHTS              64

// Attributes
in vec3 vertexPosition;
in vec2 vertexTexCoord;
in vec3 vertexNormal;
in vec4 vertexColor;

struct Light {
    int enabled;
    float falloff;
    vec3 position;
    vec4 color;
};

uniform Light lights[MAX_LIGHTS];
uniform int numLights;

// Standard Uniforms
uniform mat4 mvp;
uniform mat4 matNormal;

// Outputs to Fragment Shader
out vec2 fragTexCoord;
out vec4 fragColor; // Passing the vertex color through
out float fragBrightness;

void main()
{
    // 1. Calculate normal and light direction
    vec3 normal = normalize((matNormal * vec4(vertexNormal, 0.0)).xyz);
    vec3 lightDir = vec3(0.2, 1.0, 0.3); // Pointing straight down

    // 2. Calculate diffuse and ambient lighting
    float diffuse = max(dot(normal, lightDir), 0.0);
    float ambient = 0.2;

    // 3. Output our values to the next step
    fragBrightness = clamp(ambient + diffuse, 0.0, 1.0);
    fragColor = vertexColor;
    fragTexCoord = vertexTexCoord;

    // 4. Output final vertex position
    gl_Position = mvp * vec4(vertexPosition, 1.0);
}

