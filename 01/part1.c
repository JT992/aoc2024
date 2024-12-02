#include <stdio.h>
#include <stdlib.h>

#define MAX_STRING_LEN 20

/* test */
/* #define FILE_NAME "test.txt" */
/* #define FILE_LINES 6 */

/* actual */
#define FILE_NAME "input.txt"
#define FILE_LINES 1000

int comp(const void *elem1, const void *elem2) {
  int f = *((int *)elem1);
  int s = *((int *)elem2);
  if (f > s)
    return 1;
  if (f < s)
    return -1;
  return 0;
}

int fill_lists(int *first_list, int *second_list) {
  FILE *file = fopen(FILE_NAME, "r");
  if (!file)
    return 0;
  char line[MAX_STRING_LEN];
  int i = -1;
  while (fgets(line, MAX_STRING_LEN, file) && ++i < FILE_LINES) {
    sscanf(line, "%d%d", &first_list[i], &second_list[i]);
  }
  return 1;
}

long long get_dist(int *first_list, int *second_list) {
  long long distance = 0;
  for (int i = 0; i < FILE_LINES; i++) {
    distance += abs(first_list[i] - second_list[i]);
  }
  return distance;
}

int main() {
  int *first_list = malloc(sizeof(int) * FILE_LINES);
  int *second_list = malloc(sizeof(int) * FILE_LINES);
  if (!first_list || !second_list)
    return 1;

  fill_lists(first_list, second_list);

  qsort(first_list, FILE_LINES, sizeof(int), comp);
  qsort(second_list, FILE_LINES, sizeof(int), comp);

  printf("%lld\n", get_dist(first_list, second_list));

  free(first_list);
  free(second_list);
  return 0;
}
