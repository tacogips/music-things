/* SPDX-License-Identifier: MIT */
#ifndef MUSIC_THINGS_STIMULUS_H
#define MUSIC_THINGS_STIMULUS_H
#include "fnv.h"
#include <math.h>
#include <string.h>
#define AW_FRAMES 8192
#define PITCH_SAMPLES 11025
static inline uint32_t mt_xorshift32(uint32_t *s) {
    uint32_t x = *s; x ^= x << 13; x ^= x >> 17; x ^= x << 5; *s = x; return x;
}
static inline double mt_parabolic(double p) {
    double x = 2.0 * p - 1.0; return 4.0 * x * (1.0 - fabs(x));
}
static inline void aw_stimulus(float sample_rate, float *out_l, float *out_r) {
    double l = 0.0, r = 0.0, il = 220.0 / (double)sample_rate, ir = 330.0 / (double)sample_rate;
    uint32_t s = UINT32_C(0x12345678);
    for (size_t n = 0; n < AW_FRAMES; ++n) {
        double a = 0.0, b = 0.0;
        if (n == 0) { a = 0.9; b = -0.5; }
        else if (n >= 256 && n < 4096) {
            double amp = 0.05 + 1.15 * ((double)(n - 256) / 3840.0);
            a = amp * mt_parabolic(l); b = amp * mt_parabolic(r);
            l += il; if (l >= 1.0) l -= 1.0;
            r += ir; if (r >= 1.0) r -= 1.0;
        } else if (n >= 4096 && n < 6144) {
            a = ((double)mt_xorshift32(&s) / 4294967296.0 * 2.0 - 1.0) * 0.5;
            b = ((double)mt_xorshift32(&s) / 4294967296.0 * 2.0 - 1.0) * 0.5;
        }
        out_l[n] = (float)a; out_r[n] = (float)b;
    }
}
static inline void pitch_stimulus(float *out) {
    double ph = 0.0; uint32_t s = UINT32_C(0x9E3779B9);
    for (size_t n = 0; n < PITCH_SAMPLES; ++n) {
        double f = 220.0 + 110.0 * ((double)n / 11025.0);
        double q = 2.0 * ph; if (q >= 1.0) q -= 1.0;
        double noise = (double)mt_xorshift32(&s) / 4294967296.0 * 2.0 - 1.0;
        out[n] = (float)(0.3 * mt_parabolic(ph) + 0.15 * mt_parabolic(q) + 0.05 * noise);
        ph += f / 44100.0; if (ph >= 1.0) ph -= 1.0;
    }
}
static inline uint64_t f32_buffer_fnv(const float *a, size_t n) {
    uint64_t h = FNV1A64_OFFSET;
    for (size_t i = 0; i < n; ++i) {
        uint32_t bits; unsigned char bytes[4]; memcpy(&bits, &a[i], sizeof bits);
        for (size_t j = 0; j < 4; ++j) bytes[j] = (unsigned char)(bits >> (8 * j));
        h = fnv1a64_update(h, bytes, 4);
    }
    return h;
}
#endif
