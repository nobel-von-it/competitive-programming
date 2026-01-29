#include <ctype.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int solution(int number) {
  int res = 0;
  for (int i = 0; i < number; ++i)
    if (i % 3 == 0 || i % 5 == 0)
      res += i;

  return res;
}

// int find_odd(size_t l, const int a[l]) {
//   // for (int i = 0; i < l; ++i) {
//   //   int oddc = 0;
//   //   for (int j = 0; j < l; ++j) {
//   //     if (a[j] == a[i])
//   //       oddc += 1;
//   //   }
//   //   if (oddc % 2 == 0)
//   //     return a[i];
//   // }
//   // return 0;
//
//   int odd = 0;
//   for (int i = 0; i < l; ++i)
//     odd ^= a[i];
//   return odd;
// }

//  do not allocate memory for return string
//  assign the value to the pointer "result"

int len(const char *s) {
  int len = 0;
  while (*s != '\0') {
    len++;
    s++;
  }
  return len;
}

void memerr() {
  perror("Failed memory allocation");
  exit(EXIT_FAILURE);
}

char **csplit(const char *s, char d, int *size) {
  int i, j, k, l = len(s), prts = 1;

  for (i = 0; i < l; ++i)
    if (s[i] == d)
      prts++;

  char **res = (char **)malloc(prts * sizeof(char *));
  if (res == NULL)
    memerr();

  int strt = 0, pidx = 0;

  for (i = 0; i <= l; ++i) {
    if (s[i] == d || s[i] == '\0') {
      int pl = i - strt;
      res[pidx] = (char *)malloc((pl + 1) * sizeof(char));
      if (res[pidx] == NULL)
        memerr();

      for (j = 0; j < pl; ++j)
        res[pidx][j] = s[strt + j];
      res[pidx][pl] = '\0';

      strt = i + 1;
      pidx++;
    }
  }

  *size = prts;
  return res;
}

void free_splt(char **prts, int size) {
  for (int i = 0; i < size; ++i)
    free(prts[i]);
  free(prts);
}

void rev(char *s) {
  int l = len(s);
  for (int i = 0; i < l; ++i) {
    int tmp = s[i];
    s[i] = s[l - i];
    s[l - i] = tmp;
  }
}

void spin_words(const char *s, char *r) {
  int size;
  char **wrds = csplit(s, ' ', &size);
  for (int i = 0; i < size; ++i) {
    if (len(wrds[i]) > 4)
      rev(wrds[i]);
  }
}

int next(int n) {
  int res = 1;
  do {
    res *= (n % 10);
    n /= 10;
  } while (n > 10);
  return res;
}

int persistence(int n) {
  int res = 0;
  while (n > 10) {
    n = next(n);
    res++;
  }
  return res;
}

//  do not allocate memory for the result
//  write to pre-allocated pointer *camel

void to_camel_case(const char *src, char *r) {
  if (strlen(src) == 0) {
    r[0] = '\0';
    return;
  }
  const char *dels = "-_";
  int w = 0;
  char *s = strdup(src);
  char *t = strtok((char *)s, dels);
  if (t == NULL) {
    puts("Dels incorrect");
    free(s);
    return;
  }

  while (t != NULL) {
    int l = strlen(t);
    for (int i = 0; i < l; ++i) {
      if (i == 0 && w != 0)
        r[w++] = toupper(t[i]);
      else if (w == 0)
        r[w++] = t[i];
      else
        r[w++] = tolower(t[i]);
    }
    t = strtok(NULL, dels);
  }
  r[w] = '\0';
  free(s);
}

int main() {
  const char *t1 = "the-bEst_DvE";
  char *r = (char *)malloc(strlen(t1) - 2 * sizeof(char));
  to_camel_case(t1, r);
  printf("%s\n", r);
}

long long *tribonacci(const long long signature[3], size_t n) {
  long long *res = (long long *)malloc(n * sizeof(long long));
  size_t i;

  if (n < 4) {
    for (i = 0; i < n; ++i) {
      res[i] = signature[i];
    }
    return res;
  }
  for (size_t i = 0; i < n; ++i) {
  }

  return res;
}
