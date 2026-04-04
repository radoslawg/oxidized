#version 330

// Attributes
in vec3 vertexPosition;
in vec2 vertexTexCoord;
in vec3 vertexNormal;
in vec4 vertexColor;


// Standard Uniforms
uniform mat4 mvp;
uniform mat4 matNormal;
uniform mat4 matModel; // NEW: We need this to get the world-space position

// Outputs to Fragment Shader
out vec2 fragTexCoord;
out vec4 fragColor; // Passing the vertex color through
out float fragBrightness;
out vec3 fragPosition; // NEW: Send position to fragment shader
out vec3 fragNormal;   // NEW: Send normal to fragment shader

void main()
{
    // 1. Calculate normal and light direction
    vec3 normal = normalize((matNormal * vec4(vertexNormal, 0.0)).xyz);
    vec3 lightDir = vec3(0.2, 1.0, 0.3); // Pointing straight down

    // 2. Calculate diffuse and ambient lighting
    float diffuse = max(dot(normal, lightDir), 0.0);
    float ambient = 0.0;

    // 3. Output our values to the next step
    fragBrightness = clamp(ambient + diffuse, 0.0, 1.0);
    fragColor = vertexColor;
    fragTexCoord = vertexTexCoord;

    fragPosition = vec3(matModel * vec4(vertexPosition, 1.0));
    fragNormal = normal;

    gl_Position = mvp * vec4(vertexPosition, 1.0);
}

