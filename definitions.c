#include <unistd.h>
#include <stddef.h>
#include <stdlib.h>
#include <stdio.h>

extern char memory[8 * 64 * 1024];
//extern char memory[1024];

typedef unsigned int __wasi_fd_t;
typedef size_t __wasi_size_t;

typedef struct __wasi_ciovec_t
{
    int string_offset;
    size_t buf_len;
} __wasi_ciovec_t;

ssize_t fd_write(__wasi_fd_t fd, uintptr_t iovs_offset, const size_t iov_len, __wasi_size_t buf_count)
{
    printf("fd_write called\n fd: %u\n  iovs: %lu\n  iov_len: %zu\n  buf_count: %zu\n", fd, iovs_offset, iov_len, buf_count);

    ssize_t total_written = 0;
    const char iovs = memory[iovs_offset];
    printf("address of iovs: %p\n", &iovs);
    printf("memory: %p\n", &memory[0]);

    for (size_t i = 0; i < iov_len; ++i)
    {
        int offset = i * sizeof(__wasi_ciovec_t);
        int* string_addr = (int*) &memory[iovs_offset + offset ];
        printf("string_addr: %d\n", *string_addr);

        int length = memory[iovs_offset + offset + 4] ;
        printf("length: %d\n", length);


        ssize_t written = write(fd, &memory[*string_addr], length);
        if (written < 0)
        {
            return -1; // Write error occurred
        }
        total_written += written;
        memory[buf_count] = total_written;
    }

    printf("total_written: %zu\n", total_written);
    return 0;
}

typedef unsigned int __wasi_exitcode_t;

void proc_exit(__wasi_exitcode_t exit_code)
{
    exit(exit_code);
}
