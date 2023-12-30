#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[])
{

    volatile double numerator = atoi(argv[1]);
    volatile double denominator = atoi(argv[2]);

    volatile double result = numerator / denominator;

    printf("Result: %d\n", result);

    return 0;
}
