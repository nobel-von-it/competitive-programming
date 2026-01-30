#include <stdlib.h>
int *buildArray(int *nums, int numsSize, int *returnSize) {
  *returnSize = numsSize;
  int *res = (int *)malloc(numsSize * sizeof(int));

  for (int i = 0; i < numsSize; ++i)
    res[i] = nums[nums[i]];

  return res;
}
