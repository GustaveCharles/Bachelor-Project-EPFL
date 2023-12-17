#include <unistd.h>
#include <stddef.h>
#include <stdlib.h>
#include <stdio.h>

extern char memory[8*65536];

typedef unsigned int __wasi_fd_t;
typedef size_t __wasi_size_t;

typedef struct __wasi_ciovec_t {
    const void *buf;
    size_t buf_len;
} __wasi_ciovec_t;


ssize_t fd_write(__wasi_fd_t fd,  uintptr_t iovs_offset, const size_t iov_len, __wasi_size_t buf_count)
{
    printf("fd_write called\n fd: %u\n  iovs: %lu\n  iov_len: %zu\n  buf_count: %zu\n", fd, iovs_offset, iov_len, buf_count);

    ssize_t total_written = 0;
    const __wasi_ciovec_t* iovs = (__wasi_ciovec_t*) &memory[iovs_offset];
    printf("iovs: %p\n", iovs);
    printf("iovs[0].buf: %p\n", &iovs[0].buf);
    printf("memory: %p\n", &memory[0]);

    for (size_t i = 0; i < iov_len; ++i)
    {
        const char* string_addr = iovs[i].buf;
        int length = iovs[i].buf_len;
        
        printf("string_addr: %s\n", string_addr);   
        printf("length: %d\n", length);

        ssize_t written = write(fd, string_addr, length);
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
