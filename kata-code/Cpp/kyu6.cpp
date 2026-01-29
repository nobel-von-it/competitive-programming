int solution(int number) {
  int res = 0;
  for (int i = 0; i < number; ++i)
    if (i % 3 == 0 || i % 5 == 0)
      res += i;
  return res;
}
