#version 450
layout(location = 0) in vec2 fragTexCoord;
layout(location = 1) in vec3 fragColor;

layout(set = 0, binding = 0) uniform sampler2D fontTexture;

layout(location = 0) out vec4 outColor;

void main() {
    float alpha = texture(fontTexture, fragTexCoord).r;
    outColor = vec4(fragColor, alpha);
}
