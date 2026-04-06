#version 100

// Attributes
attribute vec3 vertexPosition;
attribute vec2 vertexTexCoord;
attribute vec3 vertexNormal;
attribute vec4 vertexColor; // We need this back!

// Standard Uniforms
uniform mat4 mvp;
uniform mat4 matNormal;

// Outputs to Fragment Shader
varying vec2 fragTexCoord;
varying vec4 fragColor; // Passing the vertex color through
varying float fragBrightness;

void main()
{
    // 1. Calculate normal and light direction
    vec3 normal = normalize((matNormal * vec4(vertexNormal, 1.0)).xyz);
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

