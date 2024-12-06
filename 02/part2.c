#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STRING_LEN 80
#define MAX_LEVELS 8

/* test */
/* #define FILE_NAME "test.txt" */

/* actual */
#define FILE_NAME "input.txt"

typedef enum {
  VerySafe = 0,
  KindaSafe = 1,
  NotSafe = 2,
} Safeness;

int intcmp(int a, int b) { return (a > b) - (a < b); }

int init_direction(int *head) {
  int ab = intcmp(head[0], head[1]), bc = intcmp(head[1], head[2]),
      ac = intcmp(head[0], head[2]), bd = intcmp(head[1], head[3]),
      cd = intcmp(head[2], head[3]);
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

Safeness window_edge_safeness(int *head, int direction) {
  printf("%d %d ", head[0], head[1]);
  return !is_pair_safe(head[0], head[1], direction);
}

Safeness window_safeness(int *head, int direction) {
  if (is_pair_safe(head[0], head[1], direction)) {
    return VerySafe;
  } else if (is_pair_safe(head[0], head[2], direction)) {
    return KindaSafe;
  } else {
    return NotSafe;
  }
}

int main() {
  FILE *file = fopen(FILE_NAME, "r");
  if (!file)
    return 0;
  char strline[MAX_STRING_LEN], *strhead;
  int intline[MAX_LEVELS + 1], *inthead;
  int i, level, direction, report_safeness, value_safeness, advancement,
      number_safe = 0;
  while (fgets(strline, MAX_STRING_LEN, file)) {
    strhead = strline;
    inthead = intline;
    // SAFETY: assume that MAX_LEVELS is correct
    for (i = 0; (level = strtol(strhead, &strhead, 10)) != 0; i++) {
      printf("%d ", level);
      intline[i] = level;
    }
    printf(":: ");
    direction = init_direction(inthead);
    value_safeness = window_edge_safeness(inthead, direction);
    report_safeness = 2 - value_safeness;
    advancement = value_safeness + 1;
    inthead += advancement;
    i -= advancement;
    while (i > 2) {
      /* printf("%d ", inthead[0]); */
      value_safeness = window_safeness(inthead, direction);
      report_safeness -= value_safeness;
      advancement = value_safeness + 1;
      inthead += advancement;
      i -= advancement;
    }
    /* if (i == 1) { */
    /*   printf("!"); */
    /*   // we've got one more to check, */
    /*   // but we do need to move the head back before that */
    /*   i++; */
    /*   inthead--; */
    /* } */
    if (i == 2) {
      printf("! ");
      // we've still got one more to check
      report_safeness -= window_edge_safeness(inthead, direction);
    }
    printf(":: %d\n", report_safeness);
    number_safe += (report_safeness > 0);
  }
  printf("%d\n", number_safe);
  return 0;
}
