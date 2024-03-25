#include <stdarg.h>

#include <fcntl.h>
#include <sys/types.h>

int open64(const char *filename, int flags, ...) {
    if ((flags & O_CREAT) || (flags & O_TMPFILE) == O_TMPFILE) {
        mode_t mode = 0;
        va_list args;
        
        va_start(args, flags);
        mode = va_arg(args, mode_t);
        va_end(args);
        
        return open(filename, flags, mode);
    }
    
    return open(filename, flags);
}