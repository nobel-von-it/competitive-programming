#include <stdio.h>

#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

bool isStrictlyPalindromic(int n)
{
    for (int b = 2; b < n - 2; b++) {
        char buf[256];
        int tmp = n;
        int in = 1;
        int ib = 0;
        while (tmp > 0) {
            buf[ib++] = (tmp % b) + '0';
            in *= 10;
            tmp /= b;
        }
        for (int i = 0; i < ib / 2; i++) {
            printf("cmp bi %c and bib %c\n", buf[i], buf[ib - i]);
            if (buf[i] != buf[ib - i - 1]) return false;
        }
        buf[ib++] = '\0';
        printf("%d base %d is %s\n", n, b, buf);
        memset(buf, 0, sizeof(buf));
    }
    return true;
}

int main(void)
{
    printf("res: %d\n", isStrictlyPalindromic(4));
    return 0;
}
