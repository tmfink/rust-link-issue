#include <stddef.h>
#include <kvm.h>

size_t foo() {
    kvm_t *kd = kvm_openfiles(NULL, NULL, NULL, 0, NULL);
    return (size_t) kd;
}
