#version 300 es

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;

uniform mat4 mvpMatrix;

out vec4 vertexColor;

void main() {
    vertexColor = color;
    gl_Position = mvpMatrix * vec4(position, 1.0);
}