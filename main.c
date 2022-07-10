#include <stdio.h>

#include "dyn.h"
#include "strconv.h"

// typedef struct {
//     const char* key;
//     void* value;
// } DynMapEntry;
//
// typedef struct {
//     DynMapEntry* entries;
//     size_t len;
// } DynMap;
//
// DynMap DynMap_new() {
//     DynMap dynMap;
//     dynMap.entries = NULL;
//     dynMap.len = 0;
//     return dynMap;
// }
//
// void pushEntry(DynMap* dynMap, const char* key, void* value) {
//     size_t allocSize = sizeof(DynMapEntry) * (dynMap->len + 1);
//     DynMapEntry* entries = (DynMapEntry*)malloc(allocSize);
//     if (dynMap->len != 0) {
//         memcpy(entries, dynMap->entries, sizeof(DynMapEntry) * dynMap->len);
//     }
//     DynMapEntry* insertInto = &entries[dynMap->len];
//     insertInto->key = key;
//     insertInto->value = value;
//
//     dynMap->entries = entries;
//     dynMap->len++;
// }
//

int main() {
    char strbuf[10];
    string_t string = string_fromBuf(strbuf, 10);
    strconv_uitoa(11111111111, string);
    printf("%s", strbuf);

//    dyn_t object = dyn_StringFromCstr("foobar");
//    printf("%s", dyn_asString(object).success ? "true" : "false");
//    dyn_free(object);
}
