#include <stdio.h>

struct point {
  int x;
  int y;
};

typedef struct point point_t;

enum color { RED, GREEN };

static int add(int a, int b) {
  return a + b;
}

int main(void) {
  printf("%d\n", add(1, 2));
  return 0;
}
