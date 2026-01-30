#include <stdlib.h>

int *getConcatenation(int *nums, int numsSize, int *returnSize) {
  *returnSize = numsSize * 2;
  int *res = (int *)malloc(numsSize * 2 * sizeof(int));
  for (int i = 0; i < numsSize; ++numsSize) {
    res[i] = nums[i];
    res[i + numsSize] = nums[i];
  }
  return res;
}
