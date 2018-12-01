#version 330 core

in Vertex_Data {
    vec3 color;
} IN;

void main() {
    gl_FragColor = vec4(IN.color, 1.0);
}