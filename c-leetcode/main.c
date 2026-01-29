#include <stdio.h>

#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

int numJewelsInStones(char *jewels, char *stones)
{
    int res = 0, jl = strlen(jewels), sl = strlen(stones);
    for (int i = 0; i < jl; i++) {
        for (int j = 0; j < sl; j++) {
            if (jewels[i] == stones[j]) res++;
        }
    }
    return res;
}

int main(void) { return 0; }
