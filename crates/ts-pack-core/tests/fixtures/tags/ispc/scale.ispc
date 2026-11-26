struct Vec3 {
  float x, y, z;
};

typedef float real;

enum Mode { FAST, SLOW };

export void scale(uniform float vin[], uniform float vout[], uniform int count) {
  foreach (i = 0 ... count) {
    vout[i] = vin[i] * 2;
  }
}
