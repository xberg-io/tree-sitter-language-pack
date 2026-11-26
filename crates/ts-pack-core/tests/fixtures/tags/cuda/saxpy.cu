#include <cstdio>

#define BLOCK 256

struct Params {
  float a;
  int n;
};

__global__ void saxpy(Params p, const float *x, float *y) {
  int i = blockIdx.x * blockDim.x + threadIdx.x;
  if (i < p.n) {
    y[i] = p.a * x[i] + y[i];
  }
}

__device__ float scale(float v) {
  return v * 2.0f;
}

int main() {
  Params p = {2.0f, 1024};
  saxpy<<<4, BLOCK>>>(p, nullptr, nullptr);
  printf("%f\n", scale(1.0f));
  return 0;
}
