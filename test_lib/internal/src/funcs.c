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
