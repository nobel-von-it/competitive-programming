

int finalValueAfterOperations(char **operations, int operationsSize) {
  int result = 0;
  for (int i = 0; i < operationsSize; ++i) {
    switch (operations[i][1]) {
    case '-':
      result--;
      break;
    case '+':
      result++;
      break;
    }
  }
  return result;
}
