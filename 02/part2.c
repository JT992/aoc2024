#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STRING_LEN 80
#define MAX_LEVELS 8

/* test */
/* #define FILE_NAME "test.txt" */

/* actual */
#define FILE_NAME "input.txt"

#define NEXT_INT_ON_LINE() strtol(head, &head, 10)

typedef struct {
  int a, b, c, z;
} Window;

typedef enum {
  VerySafe = 0,
  KindaSafe = 1,
  NotSafe = 2,
} Safeness;

int slide_window(Window *window, int next) {
  if (!next)
    return 0;
  window->z = window->a;
  window->a = window->b;
  window->b = window->c;
  window->c = next;
  return 1;
}

int intcmp(int a, int b) { return (a > b) - (a < b); }

int init_direction(Window window, int d) {
  int ab = intcmp(window.a, window.b), bc = intcmp(window.b, window.c),
      ac = intcmp(window.a, window.c), bd = intcmp(window.b, d),
      cd = intcmp(window.c, d);
  int sum = ab + bc + ac + bd + cd;
  /* printf("%d\n", sum); */
  if (sum > 0) {
    return 1;
  } else if (sum < 0) {
    return -1;
  } else {
    return 0;
  }
}

int is_pair_safe(int prev, int cur, int direction) {
  int diff = abs(cur - prev);
  return (intcmp(prev, cur) == direction) && (diff >= 1 && diff <= 3);
}

Safeness window_safeness(Window window, int direction) {
  if (is_pair_safe(window.a, window.b, direction)) {
    return VerySafe;
  } else if (is_pair_safe(window.a, window.c, direction) ||
             (is_pair_safe(window.b, window.c, direction) &&
              is_pair_safe(window.z, window.b, direction))) {
    return KindaSafe;
  } else {
    return NotSafe;
  }
}

int main() {
  FILE *file = fopen(FILE_NAME, "r");
  if (!file)
    return 0;
  char line[MAX_STRING_LEN];
  char *head;
  int direction, safeness, first_d, number_safe = 0;
  Window window;
  while (fgets(line, MAX_STRING_LEN, file)) {
    head = line;
    safeness = 2;
    window = (Window){NEXT_INT_ON_LINE(), NEXT_INT_ON_LINE(),
                      NEXT_INT_ON_LINE(), -10 /* just a sentinel */};
    first_d = NEXT_INT_ON_LINE();
    /* printf("%d %d %d %d: ", window.a, window.b, window.c, first_d); */
    direction = init_direction(window, first_d);
    safeness -= window_safeness(window, direction);
    slide_window(&window, first_d);
    while ((safeness -= window_safeness(window, direction)) > 0 &&
           slide_window(&window, NEXT_INT_ON_LINE()))
      ;
    number_safe += (safeness > 0);
    /* printf("and its safeness is %d\n", safeness); */
  }
  printf("%d\n", number_safe);
  return 0;
}
