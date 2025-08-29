#include <stdio.h>

#include <stdlib.h>
#include <string.h>

int *transformArray(int *nums, int numsSize, int *returnSize)
{
    int *res = malloc(sizeof(int) * numsSize);
    *returnSize = numsSize;

    int li = 0;
    int ri = numsSize - 1;
    for (int i = 0; i < numsSize; i++) {
        if (nums[i] % 2 == 0)
            res[li++] = 0;
        else
            res[ri--] = 1;
    }

    return res;
}

int main(void)
{
    int nums[] = {4, 3, 2, 1};
    int size = 0;
    int *res = transformArray(nums, 4, &size);

    for (int i = 0; i < size; i++) {
        printf("%d, ", res[i]);
    }

    free(res);
}
