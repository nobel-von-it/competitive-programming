#include <ctype.h>
#include <limits.h>
#include <math.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>

size_t get_count(const char *str) {
  size_t count = 0;
  for (int i = 0; str[i] != '\0'; i++)
    if (str[i] == 'a' || str[i] == 'e' || str[i] == 'i' || str[i] == 'o' ||
        str[i] == 'u')
      count++;
  return count;
}

unsigned long long square_digits(unsigned n) {
  unsigned long long res = 0;
  unsigned long long idx = 1;

  while (n != 0) {
    int dig = n % 10;
    res += dig * dig * idx;
    idx *= (dig > 3) ? 100 : 10;
    n /= 10;
  }

  return res;
}

bool is_square(int n) { return ((int)sqrt(n) == sqrt(n)); }

bool IsIsogram(const char *s) {
  char *p = (char *)s;
  for (; *p != '\0'; ++p) {
  }
  int l = p - s, i, j;
  for (i = 0; i < l; ++i) {
    char cur = tolower(s[i]);
    for (j = i + 1; j < l; ++j) {
      if (tolower(s[j]) == cur)
        return false;
    }
  }

  return true;
}

bool xo(const char *s) {
  int x = 0, o = 0;
  for (char *p = (char *)s; *p != '\0'; ++p) {
    if (tolower(*p) == 'x')
      x++;
    if (tolower(*p) == 'o')
      o++;
  }
  return x == o;
}

char *to_jaden_case(char *jc, const char *s) {
  // write to jaden_case and return it
  *jc = '\0';
  return jc;
}

ssize_t find_short(const char *s) {
  char *t = strtok((char *)s, " ");

  ssize_t res = 100000;
  while (t != NULL) {
    int tl = strlen(t);
    if (res > tl)
      res = tl;
    t = strtok(NULL, " ");
  }
  return res;
}

long sum_two_smallest_numbers(size_t n, const int a[n]) {
  int f = INT_MAX, s = INT_MAX;
  size_t i, fi;
  for (i = 0; i < n; ++i)
    if (a[i] < f) {
      fi = i;
      f = a[i];
    }
  for (i = 0; i < n; ++i)
    if (a[i] < s && fi != i)
      s = a[i];
  return (long)f + s;
}
