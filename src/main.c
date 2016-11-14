#include <stdio.h>

int run(int val, void* data, int (*calc)(int, void*)) {
    printf("%d ... %p ... %p\n", val, data, calc);
    return calc(val, data);
}
