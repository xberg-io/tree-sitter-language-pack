#define LANES 8
#define SQUARE(x) ((x) * (x))

struct Vec3 {
  float x, y, z;
};

union Bits {
  float f;
  int i;
};

typedef float real;

enum Mode { FAST, SLOW };

uniform int call_count = 0;
static const uniform float SCALE = 2.0f;

float length2(Vec3 v);

float length2(Vec3 v) {
  return SQUARE(v.x) + SQUARE(v.y) + SQUARE(v.z);
}

static inline real twice(real value) {
  return value * SCALE;
}

export void scale(uniform float vin[], uniform float vout[], uniform int count) {
  foreach (i = 0 ... count) {
    float scaled = twice(vin[i]);
    vout[i] = scaled;
  }
  call_count += 1;
}

task void scale_task(uniform float data[], uniform int span) {
  uniform int start = taskIndex * span;
  data[start] = length2((Vec3){1, 2, 3});
}

export void launch_all(uniform float data[], uniform int span) {
  launch[4] scale_task(data, span);
}
