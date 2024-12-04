#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STRING_LEN 80
#define MAX_LEVELS 8

/* test */
/* #define FILE_NAME "test.txt" */

/* actual */
#define FILE_NAME "input.txt"

int intcmp(int a, int b) { return (a > b) - (a < b); }

int is_safe(int prev, int cur, int direction) {
  int diff = abs(cur - prev);
  return (intcmp(prev, cur) == direction) && (diff >= 1 && diff <= 3);
}

int main() {
  FILE *file = fopen(FILE_NAME, "r");
  if (!file)
    return 0;
  char line[MAX_STRING_LEN];
  char *head;
  int prev_level, current_level, direction, safe, number_safe = 0;
  while (fgets(line, MAX_STRING_LEN, file)) {
    head = line;
    prev_level = strtol(line, &head, 10);
    current_level = strtol(head, &head, 10);
    direction = intcmp(prev_level, current_level);
    if (!is_safe(prev_level, current_level, direction)) {
      continue;
    }
    prev_level = current_level;
    while ((current_level = strtol(head, &head, 10)) &&
           (safe = is_safe(prev_level, current_level, direction))) {
      prev_level = current_level;
    }
    // note: no if-check needed!
    // if the whole line is safe, `safe` is 1, we can just add it!
    // if not, `safe` is 0, we can just add it anyway!
    number_safe += safe;
  }
  printf("%d\n", number_safe);
  return 0;
}
