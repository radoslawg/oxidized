#version 330

// Attributes
in vec3 vertexPosition;
in vec2 vertexTexCoord;
in vec3 vertexNormal;

// Standard Uniforms
uniform mat4 mvp;
uniform mat4 matNormal;


// Outputs to Fragment Shader
out vec2 fragTexCoord;
out vec3 fragBrightness; // Passes RGB brightness instead of a single float

void main()
{
    // Transform normal to world space
    vec3 normal = normalize((matNormal * vec4(vertexNormal, 1.0)).xyz);

    vec3 lightDir = normalize(vec3(0.3, 1.0, 0.2));

    float diffuse = max(dot(normal, lightDir), 0.0);

    float ambient = 0.3;

    // Output the final RGB brightness
    fragBrightness = clamp(vec3(diffuse, diffuse, diffuse) + ambient, 0.0, 1.0);

    // Pass along the UVs and position
    fragTexCoord = vertexTexCoord;
    gl_Position = mvp * vec4(vertexPosition, 1.0);
}
