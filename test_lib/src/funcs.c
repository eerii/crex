#include <stdio.h>

void test() { printf("hi from test()"); }

void test_args(int a, char b) { printf("the args are %d and %c", a, b); }

unsigned long test_ret() { return 4; }
