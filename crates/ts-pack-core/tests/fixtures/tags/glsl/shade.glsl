#version 330 core

#define PI 3.14159

uniform mat4 model;
in vec3 position;
out vec4 color;

layout(std140) uniform Camera {
  mat4 view;
} camera;

struct Light {
  vec3 dir;
  float power;
};

float lambert(vec3 n, Light l) {
  float d = max(dot(n, l.dir), 0.0);
  return d * l.power;
}

void main() {
  Light sun = Light(vec3(0.0, 1.0, 0.0), 1.0);
  color = vec4(vec3(lambert(position, sun)), 1.0);
}
