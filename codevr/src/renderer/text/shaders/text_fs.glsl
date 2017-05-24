in vec2 pos;
out vec4 color;

layout (location = 0) in vec2 iUV;
layout (location = 0) out vec4 oColor;

layout(set = 0, binding = 0) uniform Block {
    sampler2D msdf;
} uniforms;

uniform sampler2D msdf;
uniform vec4 bgColor;
uniform vec4 fgColor;

float median(float r, float g, float b) {
    return max(min(r, g), min(max(r, g), b));
}

void main() {
    vec3 sample = texture(msdf, pos).rgb;
    float sigDist = median(sample.r, sample.g, sample.b) - 0.5;
    float opacity = clamp(sigDist/fwidth(sigDist) + 0.5, 0.0, 1.0);
    oColor = mix(bgColor, fgColor, opacity);
}