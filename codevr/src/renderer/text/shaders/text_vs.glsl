layout(set = 0, binding = 0) uniform Block {
    mat4 transform;
} uniforms;

layout (location = 0) in vec2 iUV;
layout (location = 0) out vec4 oColor;

void main() {
  outVert = transform * iVertex;
}