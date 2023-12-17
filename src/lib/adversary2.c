#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    // if (argc != 3) {
    //     printf("Usage: %s <numerator> <denominator>\n", argv[0]);
    //     return 1;
    // }

    volatile double numerator = atof(argv[1]);
    volatile double denominator = atof(argv[2]);

    volatile double result = numerator / denominator;

    printf("Result: %lf\n", result);

    return 0;
}
