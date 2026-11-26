#include <stdio.h>
#include <stdlib.h>

#define MAX_SHAPES 16
#define SQUARE(x) ((x) * (x))

struct point {
  int x;
  int y;
};

union value {
  int i;
  double d;
};

typedef struct point point_t;

typedef struct {
  double w;
  double h;
} rect_t;

typedef struct node *node_ptr;

typedef int (*compare_fn)(const void *, const void *);

enum color { RED, GREEN = 2, BLUE };

static const int origin_x = 0;
int shape_count = 0;
static double scale_table[4];
const char *default_name = "shape";
struct node *head;
extern int external_count;

int add(int a, int b);

static int add(int a, int b) {
  int sum = a + b;
  return sum;
}

char *describe(const point_t *p) {
  static char buffer[64];
  snprintf(buffer, sizeof buffer, "(%d, %d)", p->x, p->y);
  return buffer;
}

int main(void) {
  point_t p = {1, 2};
  printf("%d %s\n", add(SQUARE(2), origin_x), describe(&p));
  return 0;
}
