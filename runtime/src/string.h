#ifndef HTTP_LANG_STRING_H
#define HTTP_LANG_STRING_H

#include <stddef.h>
#include <string.h>

#include "mem.h"

typedef struct {
    size_t len;
    char* data;
} string_t;

string_t string_new(const char* cstr) {
    string_t string;
    string.len = strlen(cstr);
    if (string.len != 0) {
        string.data = (char*) memAlloc(string.len * sizeof(char));
        memcpy(string.data, cstr, string.len);
    } else {
        string.data = NULL;
    }
    return string;
}

string_t string_fromBuf(char* buf, size_t len) {
    string_t string;
    string.data = buf;
    string.len = len;
    return string;
}

string_t string_clone(string_t string) {
    if (string.len != 0) {
        char* mem = (char*) memAlloc(string.len);
        memcpy(mem, string.data, string.len);
        string.data = mem;
    }
    return string;
}

typedef struct {
    char data[64];
} string_Fixed64;

string_Fixed64 string_fixed64(string_t string) {
    const size_t MAX_CONTENT_LEN = 64 - 1; // for \0

    string_Fixed64 fixed;
    memcpy(fixed.data, string.data, string.len > MAX_CONTENT_LEN ? MAX_CONTENT_LEN : string.len);
    if (string.len <= MAX_CONTENT_LEN) {
        memset(fixed.data + string.len, 0, MAX_CONTENT_LEN - string.len);
    }
    fixed.data[63] = 0;
    return fixed;
}

const char* string_cstr(string_t string) {
    if (string.len == 0) {
        return "";
    }

    char* mem = (char*) memAlloc(string.len + 1);
    memcpy(mem, string.data, string.len);
    mem[string.len] = '\0';
    return mem;
}

void string_free(string_t string) {
    memFree(string.data);
}

#endif // HTTP_LANG_STRING_H
