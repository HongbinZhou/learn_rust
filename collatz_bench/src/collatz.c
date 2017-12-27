int collatz_c(int n) {
  int l = 0;
  while (n != 1) {
    if (n % 2 == 0)
      n = n / 2;
    else
      n = 3 * n + 1;
    l++;
  }
  return l;
}
