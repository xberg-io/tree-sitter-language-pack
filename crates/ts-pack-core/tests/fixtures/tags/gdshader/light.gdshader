shader_type spatial;

struct Light {
  vec3 color;
};

float brightness(vec3 c) {
  return dot(c, vec3(0.3, 0.6, 0.1));
}

void fragment() {
  ALBEDO = vec3(brightness(vec3(1.0)));
}
