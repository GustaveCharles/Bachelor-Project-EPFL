#include <unistd.h>

// Recursive function to find the nth Fibonacci number
int fibonacci(int n)
{
    if (n <= 1)
        return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

int main()
{
    int n = 40; // Change this to compute a different Fibonacci number
    int result = fibonacci(n);
    char msg[] = "Fibonacci value: \n"; // Declare and initialize msg
    write(STDOUT_FILENO, msg, sizeof(msg) - 1);

    return 0;
}
