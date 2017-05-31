#version 450

layout (location = 0) in vec2 iUV;
layout (location = 0) out vec4 oColor;

layout(set = 0, binding = 1) uniform sampler2D tex;

float median(float r, float g, float b) {
    return max(min(r, g), min(max(r, g), b));
}

void main() {
    vec4 bgColor = vec4(255.0, 255.0, 255.0, 0.0);
    vec4 fgColor = vec4(255.0, 255.0, 255.0, 1.0);
    vec4 sampl = texture(tex, iUV);
    float sigDist = median(sampl.r, sampl.g, sampl.b) - 0.5;
    float opacity = clamp(sigDist / fwidth(sigDist) + 0.5, 0.0, 1.0);

    oColor = mix(bgColor, fgColor, opacity);
}