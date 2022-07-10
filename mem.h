#ifndef HTTP_LANG_MEM_H
#define HTTP_LANG_MEM_H

#include <malloc.h>
#include <stdlib.h>

void* memAlloc(size_t size) {
    void* ptr = malloc(size);
    if (ptr == NULL) {
        abort();
    }
    return ptr;
}

void memFree(void* ptr) {
    free(ptr);
}

#endif // HTTP_LANG_MEM_H
