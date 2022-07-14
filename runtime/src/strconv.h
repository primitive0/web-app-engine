#ifndef HTTP_LANG_STRCONV_H
#define HTTP_LANG_STRCONV_H

#include <stdint.h>
#include <stdio.h>

#include "string.h"

void strconv_itoa(int64_t n, string_t s) {
    char buf[s.len + 1];
    snprintf(buf, s.len + 1, "%ld", n);
    memcpy(s.data, buf, s.len);
}

void strconv_uitoa(uint64_t n, string_t s) {
    char buf[s.len + 1];
    snprintf(buf, s.len + 1, "%lu", n);
    memcpy(s.data, buf, s.len);
}

#endif // HTTP_LANG_STRCONV_H
