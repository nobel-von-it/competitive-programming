#include <stdio.h>

#include <stdlib.h>

int *pivotArray(int *nums, int numsSize, int pivot, int *returnSize)
{
    int *res = malloc(sizeof(int) * numsSize);
    *returnSize = numsSize;

    int *smls = malloc(sizeof(int) * numsSize);
    int smls_idx = 0;
    int *bigs = malloc(sizeof(int) * numsSize);
    int bigs_idx = 0;
    int *mids = malloc(sizeof(int) * numsSize);
    int mids_idx = 0;

    for (int i = 0; i < numsSize; i++) {
        if (nums[i] < pivot) smls[smls_idx++] = nums[i];
        if (nums[i] > pivot) bigs[bigs_idx++] = nums[i];
        if (nums[i] == pivot) mids[mids_idx++] = nums[i];
    }

    for (int i = 0; i < smls_idx; i++) {
        res[i] = smls[i];
    }
    for (int i = 0; i < mids_idx; i++) {
        res[i + smls_idx] = mids[i];
    }
    for (int i = 0; i < bigs_idx; i++) {
        res[i + smls_idx + mids_idx] = bigs[i];
    }

    free(smls);
    free(bigs);
    free(mids);

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
