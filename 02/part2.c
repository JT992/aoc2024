#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STRING_LEN 80
#define MAX_LEVELS 8

/* test */
#define FILE_NAME "test.txt"

/* actual */
/* #define FILE_NAME "input.txt" */

#define NEXT_INT_ON_LINE() strtol(head, &head, 10)

typedef struct {
  int prev;
  int pane[4];
} Window;

typedef enum {
  VerySafe = 0,
  KindaSafe = 1,
  NotSafe = 2,
} Safeness;

int slide_window(Window *window, int next) {
  if (!next)
    return 0;
  int *pane = window->pane;
  pane[0] = pane[1];
  pane[1] = pane[2];
  pane[2] = pane[3];
  pane[3] = next;
  return 1;
}

int intcmp(int a, int b) { return (a > b) - (a < b); }

int init_direction(Window window) {
  int ab = intcmp(window.pane[0], window.pane[1]),
      bc = intcmp(window.pane[1], window.pane[2]),
      ac = intcmp(window.pane[0], window.pane[2]),
      bd = intcmp(window.pane[1], window.pane[3]),
      cd = intcmp(window.pane[2], window.pane[3]);
  int sum = ab + bc + ac + bd + cd;
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

Safeness init_window_safeness(Window window, int direction) {
  return !is_pair_safe(window.pane[0], window.pane[1], direction);
}

Safeness final_window_safeness(Window window, int direction) {
  return !is_pair_safe(window.pane[2], window.pane[3], direction);
}

Safeness window_safeness(Window window, int direction) {
  if (is_pair_safe(window.pane[1], window.pane[2], direction)) {
    return VerySafe;
  } else if (is_pair_safe(window.pane[0], window.pane[2], direction) ||
             is_pair_safe(window.pane[1], window.pane[3], direction)) {
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
  int i, direction, safeness, number_safe = 0;
  Window window;
  while (fgets(line, MAX_STRING_LEN, file)) {
    head = line;
    safeness = 2;
    for (i = 0; i < 4; i++) {
      window.pane[i] = NEXT_INT_ON_LINE();
    }
    direction = init_direction(window);
    safeness -= init_window_safeness(window, direction);
    while ((safeness -= window_safeness(window, direction)) > 0 &&
           slide_window(&window, NEXT_INT_ON_LINE()))
      ;
    safeness -= final_window_safeness(window, direction);
    printf("%d\n", safeness);
    number_safe += (safeness > 0);
  }
  printf("%d\n", number_safe);
  return 0;
}
