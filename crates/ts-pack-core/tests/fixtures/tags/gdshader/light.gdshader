shader_type spatial;
render_mode unshaded;

#define TINT_STRENGTH 0.5
#define SCALE(v) ((v) * 2.0)

const float PI_APPROX = 3.14159;

uniform vec4 tint : source_color = vec4(1.0);
uniform sampler2D albedo_tex;

varying vec3 world_normal;

struct Light {
  vec3 color;
  float energy;
};

float brightness(vec3 c) {
  float weight = 0.3;
  return dot(c, vec3(weight, 0.6, 0.1));
}

Light make_light(vec3 color) {
  Light l;
  l.color = color;
  l.energy = 1.0;
  return l;
}

void vertex() {
  world_normal = NORMAL;
}

void fragment() {
  float values[3] = float[3](1.0, 2.0, 3.0);
  Light light = make_light(tint.rgb);
  vec3 base = texture(albedo_tex, UV).rgb * float(values.length());
  ALBEDO = vec3(brightness(base * light.color)) * SCALE(TINT_STRENGTH);
}
