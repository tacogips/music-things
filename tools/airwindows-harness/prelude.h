// SPDX-License-Identifier: MIT
#include <cstdlib>
#include <cmath>
#include <cstring>
#include <cstdio>
#include <cstdint>
#include <string>
#include <type_traits>
static_assert(sizeof(long double) == sizeof(double), "reference fixtures require long double == double");
inline uint32_t aw_seed(uint32_t seed, uint32_t lane) { uint32_t x = seed + lane * 0x9e3779b9u; x ^= x >> 16; x *= 0x7feb352du; x ^= x >> 15; x *= 0x846ca68bu; x ^= x >> 16; if (x < 16386) x += 16386; return x; }
inline uint32_t aw_state = 0;
inline void aw_harness_srand(uint32_t seed) { aw_state = aw_seed(seed, 2); }
inline int aw_harness_rand() { aw_state ^= aw_state << 13; aw_state ^= aw_state >> 17; aw_state ^= aw_state << 5; return static_cast<int>(aw_state >> 1); }
#define rand() aw_harness_rand()
#undef RAND_MAX
#define RAND_MAX 0x7fffffff
#define private public
#define protected public
