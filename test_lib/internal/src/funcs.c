#include <stdarg.h>
#include <stdio.h>

void test() { printf("hi from test()\n"); }

void test_args(int a, int b) {
  int sum = a + b;
  printf("%d * %d = %d\n", a, b, sum);
}

unsigned long test_ret() {
  printf("this number is bigger than it seems\n");
  return 4;
}

int test_variadic_valist(int n, va_list v) {
  int sum = 0;
  for (int i = 0; i < n; i++)
    sum += va_arg(v, int);
  printf("the sum is %d\n", sum);
  return sum;
}

int test_variadic(int n, ...) {
  va_list v;
  va_start(v, n);
  int sum = test_variadic_valist(n, v);
  va_end(v);
  return sum;
}
