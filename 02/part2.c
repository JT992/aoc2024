#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STRING_LEN 80
#define MAX_LEVELS 8

/* test */
#define FILE_NAME "test.txt"

/* actual */
/* #define FILE_NAME "input.txt" */

// big number to make sure non-spliced arrays are not spliced
#define NOT_SPLICED 16

int spliced_get(int *line, int index, int splicing_index) {
  if (index < splicing_index) {
    return line[index];
  } else {
    return line[index + 1];
  }
}

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

int is_safe(int prev, int cur, int direction) {
  int diff = abs(cur - prev);
  return (intcmp(prev, cur) == direction) && (diff >= 1 && diff <= 3);
}

int is_window_safe(int *head, int start_index, int direction,
                   int splicing_index) {
  return is_safe(spliced_get(head, start_index, splicing_index),
                 spliced_get(head, start_index + 1, splicing_index), direction);
}

int first_unsafe_index(int *line, int max, int direction, int splicing_index) {
  int i, result;
  for (i = 0;
       i < max && (result = is_window_safe(line, i, direction, splicing_index));
       i++)
    ;
  if (result) {
    return 0;
  } else {
    return i;
  }
}

int main() {
  FILE *file = fopen(FILE_NAME, "r");
  if (!file)
    return 0;
  char strline[MAX_STRING_LEN], *strhead;
  int intline[MAX_LEVELS];
  int i, max, level, direction, failure_index, number_safe = 0;
  while (fgets(strline, MAX_STRING_LEN, file)) {
    strhead = strline;
    // SAFETY: assume that MAX_LEVELS is correct
    for (i = 0; (level = strtol(strhead, &strhead, 10)) != 0; i++) {
      printf("%d ", level);
      intline[i] = level;
    }
    max = i;
    direction = init_direction(intline);
    if (!(failure_index =
              first_unsafe_index(intline, max, direction, NOT_SPLICED)) ||
        (!first_unsafe_index(intline, max - 1, direction, failure_index - 1) ||
         !first_unsafe_index(intline, max - 1, direction, failure_index) ||
         !first_unsafe_index(intline, max - 1, direction, failure_index + 1))) {
      printf("safe! %d\n", failure_index);
      number_safe++;
    } else {
      printf("UNSAFE?????\n");
    }
  }
  printf("%d\n", number_safe);
  return 0;
}
