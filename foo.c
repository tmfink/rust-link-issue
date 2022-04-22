#include <stddef.h>
#include <zlib.h>

size_t foo() {
    return (size_t) zlibVersion();
}
