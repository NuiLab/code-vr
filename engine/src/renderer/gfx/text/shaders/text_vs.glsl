#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;

layout(set = 0, binding = 0) uniform Data {
    mat4 world;
    mat4 view;
    mat4 proj;
} transforms;

layout (location = 0) out vec2 oUV;

void main() {
    oUV = uv;
    gl_Position = transforms.proj * transforms.view * transforms.world * vec4(position, 1.0);
}