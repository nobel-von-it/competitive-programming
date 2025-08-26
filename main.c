#include <stdio.h>

#include <stdlib.h>

int *pivotArray(int *nums, int numsSize, int pivot, int *returnSize)
{
    int *res = malloc(sizeof(int) * numsSize);
    *returnSize = numsSize;

    int less = 0, eq = 0;
    for (int i = 0; i < numsSize; i++) {
        if (nums[i] < pivot)
            less++;
        else if (nums[i] == pivot)
            eq++;
    }

    int isml = 0;
    int imid = less;
    int ibig = less + eq;

    for (int i = 0; i < numsSize; i++) {
        if (nums[i] < pivot) res[isml++] = nums[i];
        if (nums[i] == pivot) res[imid++] = nums[i];
        if (nums[i] > pivot) res[ibig++] = nums[i];
    }

    return res;
}

int main(void)
{
    int size = 0;
    int nums[] = {9, 12, 5, 10, 14, 3, 10};
    int *pa = pivotArray(nums, 7, 10, &size);
    printf("len: %d\n", size);
    for (int i = 0; i < size; i++) {
        printf("%d, ", pa[i]);
    }
    free(pa);
}
