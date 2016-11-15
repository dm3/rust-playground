#include <stdio.h>

int run(int val, void* data, int (*calc)(int, void*)) {
    return calc(val, data);
}
