#include <stdlib.h>
double *convertTemperature(double celsius, int *returnSize) {
  *returnSize = 2;
  double *res = (double *)malloc(*returnSize * sizeof(double));
  res[0] = (celsius * 9 / 5) + 32;
  res[1] = (celsius + 273.15);

  return res;
}
