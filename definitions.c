#include <unistd.h>
#include <stddef.h>
#include <stdlib.h>
#include <stdio.h>

extern char memory[100];

typedef unsigned int __wasi_fd_t;
ssize_t fd_write(__wasi_fd_t fd, const void *bufs, const size_t buf_lens, size_t buf_count)
{
    printf("fd_write called\n fd: %d\n  bufs: %p\n  buf_lens: %zu\n  buf_count: %zu\n", fd, bufs, buf_lens, buf_count);

    ssize_t total_written = 0;

    for (size_t i = 0; i < buf_lens; ++i)
    {
        int string_addr = memory[0];
        int length = memory[4];

        ssize_t written = write(fd, &memory[string_addr], length);
        if (written < 0)
        {
            return -1; // Write error occurred
        }
        total_written += written;
        memory[buf_count] = total_written;
    }
    
    return 0;
}

typedef unsigned int __wasi_exitcode_t;

void proc_exit(__wasi_exitcode_t exit_code)
{
    exit(exit_code);
}
