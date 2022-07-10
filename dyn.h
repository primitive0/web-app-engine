#ifndef HTTP_LANG_DYN_H
#define HTTP_LANG_DYN_H

#include <stdbool.h>
#include <stdint.h>

#include "string.h"

enum {
    Dyn_BYTE,
    Dyn_SHORT,
    Dyn_USHORT,
    Dyn_INT,
    Dyn_UINT,
    Dyn_LONG,
    Dyn_ULONG,
    Dyn_DOUBLE,
    Dyn_FINANCIAL,
    Dyn_BOOL,
    Dyn_STRING,
    Dyn_RUNE
};

typedef struct {
    uint64_t inner[2];
} DynData;

typedef struct {
    uint64_t type;
    DynData data;
} dyn_t;

#define BIT_CAST($from, $into, $value)                                                                                                                         \
    ((union {                                                                                                                                                  \
         $from from;                                                                                                                                           \
         $into into;                                                                                                                                           \
     }){.from = ($value)}                                                                                                                                      \
         .into)

#define DYN_GEN_IMPL($name, $type, $enumVariant)                                                                                                          \
    dyn_t dyn_new##$name($type value) {                                                                                                                        \
        dyn_t object;                                                                                                                                          \
        object.type = $enumVariant;                                                                                                                            \
        object.data = BIT_CAST($type, DynData, value);                                                                                                         \
        return object;                                                                                                                                         \
    }                                                                                                                                                          \
                                                                                                                                                               \
    typedef struct {                                                                                                                                           \
        $type value;                                                                                                                                           \
        bool success;                                                                                                                                          \
    } dyn_##$name##CastResult;                                                                                                                                 \
                                                                                                                                                               \
    dyn_##$name##CastResult dyn_as##$name(dyn_t object) {                                                                                                      \
        dyn_##$name##CastResult result;                                                                                                                        \
        if (object.type != ($enumVariant)) {                                                                                                                   \
            result.value = ($type){0};                                                                                                                         \
            result.success = false;                                                                                                                            \
        } else {                                                                                                                                               \
            result.value = BIT_CAST(DynData, $type, object.data);                                                                                              \
            result.success = true;                                                                                                                             \
        }                                                                                                                                                      \
        return result;                                                                                                                                         \
    }

DYN_GEN_IMPL(Byte, uint8_t, Dyn_BYTE)
DYN_GEN_IMPL(Short, int16_t, Dyn_SHORT)
DYN_GEN_IMPL(UShort, uint16_t, Dyn_USHORT)
DYN_GEN_IMPL(Int, int32_t, Dyn_INT)
DYN_GEN_IMPL(UInt, uint32_t, Dyn_UINT)
DYN_GEN_IMPL(Long, int64_t, Dyn_LONG)
DYN_GEN_IMPL(ULong, uint64_t, Dyn_ULONG)
DYN_GEN_IMPL(Bool, bool, Dyn_BOOL)
DYN_GEN_IMPL(Rune, uint64_t, Dyn_RUNE)
DYN_GEN_IMPL(String, string_t, Dyn_STRING)

dyn_t dyn_StringFromCstr(const char* cstr) {
    string_t string = string_new(cstr);

    dyn_t object;
    object.type = Dyn_STRING;
    object.data = BIT_CAST(string_t, DynData, string);
    return object;
}

string_t dyn_toString(dyn_t object) {
    string_t string;

    switch (object.type) {
    case Dyn_STRING:
        string = BIT_CAST(DynData, string_t, object.data);
        return string;

    case Dyn_BYTE:
        break;
    default:
        abort();
    }
}

void dyn_free(dyn_t object) {
    switch (object.type) {
    case Dyn_STRING: {
        string_t value = BIT_CAST(DynData, string_t, object.data);
        string_free(value);
        break;
    }
    default:
        break;
    }
}

#endif // HTTP_LANG_DYN_H
