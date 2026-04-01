#version 330

in vec3 vertexPosition;
in vec2 vertexTexCoord;
in vec3 vertexNormal;

uniform mat4 mvp;
uniform mat4 matNormal;

out vec2 fragTexCoord;
out vec3 fragBrightness; // NEW: Now a vec3 so R, G, and B can have different shadows!

void main()
{
    vec3 normal = normalize((matNormal * vec4(vertexNormal, 1.0)).xyz);

    // How far apart the color channels spread. Tweak this!
    float aberrationStrength = 0.15;

    // 1. Define three slightly different light directions
    // Green is pointing straight up (our base light)
    vec3 lightDirG = normalize(vec3(0.0, 1.0, 0.0));
    // Red is tilted slightly to one side
    vec3 lightDirR = normalize(vec3(aberrationStrength, 1.0, 0.0));
    // Blue is tilted slightly to the opposite side
    vec3 lightDirB = normalize(vec3(-aberrationStrength, 1.0, 0.0));

    // 2. Calculate lighting for each color channel separately
    float diffuseR = max(dot(normal, lightDirR), 0.0);
    float diffuseG = max(dot(normal, lightDirG), 0.0);
    float diffuseB = max(dot(normal, lightDirB), 0.0);

    // 3. Combine with ambient light and output
    float ambient = 0.3;
    fragBrightness = clamp(vec3(diffuseR, diffuseG, diffuseB) + ambient, 0.0, 1.0);

    fragTexCoord = vertexTexCoord;
    gl_Position = mvp * vec4(vertexPosition, 1.0);
}

