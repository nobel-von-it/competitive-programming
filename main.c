#include <stdlib.h>
#include <string.h>

#include <stdio.h>

int abs(int a)
{
    if (a < 0) return -a;
    return a;
}

int *minOperations(char *boxes, int *returnSize)
{
    int len = strlen(boxes);
    int *res = malloc(sizeof(int) * len);
    for (int i = 0; i < len; i++) {
        int minop = 0;
        for (int j = 0; j < len; j++) {
            if (boxes[j] == '1') minop += abs(i - j);
        }
        res[i] = minop;
    }
    *returnSize = len;
    return res;
}

int main(void)
{
    int size = 0;
    int *minops = minOperations("110", &size);
    printf("len: %d\n", size);
    for (int i = 0; i < size; i++) {
        printf("%d, ", minops[i]);
    }
    free(minops);
}
