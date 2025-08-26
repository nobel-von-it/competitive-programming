#include <stdio.h>

#include <stdlib.h>
#include <string.h>

char *defangIPaddr(char *address)
{
    int len = strlen(address);
    int nlen = len + 7;

    int ir = 0;
    char *res = malloc(sizeof(char) * nlen);
    for (int i = 0; i < len; i++) {
        if (address[i] == '.') {
            res[ir++] = '[';
            res[ir++] = '.';
            res[ir++] = ']';
        } else {
            res[ir++] = address[i];
        }
    }

    res[nlen - 1] = '\0';

    return res;
}

int main(void)
{
    char *daddr = defangIPaddr("1.1.1.1");
    printf("%s\n", daddr);
    free(daddr);
}
