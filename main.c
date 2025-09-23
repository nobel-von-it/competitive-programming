#include <stdio.h>

#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

int *recoverOrder(int *order, int orderSize, int *friends, int friendsSize, int *returnSize)
{
    int *res = malloc(sizeof(int) * friendsSize);
    int ri = 0;
    for (int i = 0; i < orderSize; i++) {
        for (int j = 0; j < friendsSize; j++) {
            if (order[i] == friends[j]) res[ri++] = order[i];
        }
    }

    *returnSize = friendsSize;
    return res;
}

int main(void) { return 0; }
