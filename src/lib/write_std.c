#include <unistd.h>

int main(void)
{
    char msg[] = "Hello, world!\n"; // Declare and initialize msg
    write(STDOUT_FILENO, msg, sizeof(msg) - 1);
    return 0;
}
