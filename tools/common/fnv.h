/* SPDX-License-Identifier: MIT */
#ifndef MUSIC_THINGS_FNV_H
#define MUSIC_THINGS_FNV_H
#include <stddef.h>
#include <stdint.h>
#define FNV1A64_OFFSET UINT64_C(0xcbf29ce484222325)
#define FNV1A64_PRIME UINT64_C(0x100000001b3)
static inline uint64_t fnv1a64_update(uint64_t h, const unsigned char *p, size_t n) {
    for (size_t i = 0; i < n; ++i) { h ^= p[i]; h *= FNV1A64_PRIME; }
    return h;
}
#endif
