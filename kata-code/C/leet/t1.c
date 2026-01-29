#include <stdlib.h>
int *twoSum(int *ns, int l, int t, int *r) {
  for (int i = 0; i < l; ++i) {
    for (int j = i + 1; j < l; ++j) {
      if (ns[i] + ns[j] == t) {
        int *res = (int *)malloc(2 * sizeof(int));
        res[0] = i;
        res[1] = j;
        *r = 2;
        return res;
      }
    }
  }
  *r = 0;
  return NULL;
}
