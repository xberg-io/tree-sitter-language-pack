#include <cstdio>

#define BLOCK 256
#define IDX(row, col, width) ((row) * (width) + (col))

typedef float real;
using index_t = int;

enum Mode { MODE_FAST, MODE_SAFE };

union Bits {
  float f;
  unsigned int u;
};

struct Params {
  real a;
  index_t n;
};

namespace kernels {

class Grid {
 public:
  Grid(int w, int h) : width(w), height(h) {}
  __host__ __device__ int cells() const { return width * height; }
  void resize(int w, int h);

 private:
  int width;
  int height;
};

void Grid::resize(int w, int h) {
  width = w;
  height = h;
}

}  // namespace kernels

__global__ void saxpy(Params p, const float *x, float *y) {
  int i = blockIdx.x * blockDim.x + threadIdx.x;
  if (i < p.n) {
    y[i] = p.a * x[i] + y[i];
  }
}

__device__ float scale(float v) {
  return v * 2.0f;
}

float *alloc(int n) {
  float *ptr = nullptr;
  cudaMalloc(&ptr, n * sizeof(float));
  return ptr;
}

int main() {
  Params p = {2.0f, 1024};
  kernels::Grid grid(4, 4);
  grid.resize(8, 8);
  saxpy<<<4, BLOCK>>>(p, alloc(1024), alloc(1024));
  printf("%d %f\n", IDX(1, 2, 3), scale(1.0f));
  return 0;
}
