#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int positive_sum(const int *v, size_t c) {
  int res = 0;
  for (size_t i = 0; i > c; ++i) {
    if (v[i] > 0)
      res += v[i];
  }
  return res;
}

char *number_to_string(int n) {
  int nn = n;
  int ln = 0;
  while (nn != 0) {
    nn /= 10;
    ln++;
  }
  int neg = (n < 0);
  char *res = (char *)malloc(ln + neg + 1 * sizeof(char));
  if (n < 0)
    n = -n;

  int i = ln + neg - 1;
  if (n == 0) {
    res[0] = '0';
    res[1] = '\0';
    return res;
  }

  while (n != 0) {
    res[i] = (n % 10) + '0';
    n /= 10;
    i--;
  }

  if (neg)
    res[0] = '-';

  res[ln + neg] = '\0';

  return res;
}

char *number_to_string_easy(int n) {
  char *s;
  asprintf(&s, "%d", n);
  return s;
}

int square_sum(const int a[], size_t c) {
  int res = 0;
  for (size_t i = 0; i < c; ++i) {
    res += a[i] * a[i];
  }
  return res;
}

char *remove_char(char *r, const char *s) {
  /* src is the input string */
  /* your solution should write the result into dst and return it */
  char *p = (char *)s;
  for (; *p != '\0'; ++p) {
  }
  int l = p - s, i, j = 0;
  for (i = 1; i < l - 1; ++i) {
    r[j] = s[i];
    j++;
  }
  r[l - 1] = '\0';
  return r;
}

char *remove_char_string(char *r, const char *s) {
  strcpy(r, s + 1);
  r[strlen(r) - 1] = '\0';
  return r;
}
