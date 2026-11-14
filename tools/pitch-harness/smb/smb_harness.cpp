// SPDX-License-Identifier: MIT
#include <cstdio>
#include <cstdlib>
#include <cstdint>
#include <cmath>
#include "stimulus.h"
void smbPitchShiftInit(float, long, long, float);
void smbPitchShift(size_t, int16_t *, int16_t *);
int main(int argc, char **argv) {
    if (argc != 3) return 2;
    const int config = std::atoi(argv[1]);
    if (config != 1 && config != 2) return 2;
    float input[PITCH_SAMPLES]; int16_t q[PITCH_SAMPLES], out[PITCH_SAMPLES];
    pitch_stimulus(input);
    if (f32_buffer_fnv(input, PITCH_SAMPLES) != UINT64_C(0x4ed50bb4be37bb1a)) return 3;
    for (size_t i = 0; i < PITCH_SAMPLES; ++i) {
        if (std::fabs(input[i]) > 0.5) return 4;
        q[i] = static_cast<int16_t>(static_cast<double>(input[i]) * 32768.0);
    }
    smbPitchShiftInit(config == 1 ? 1.5f : 0.75f, config == 1 ? 1024 : 2048, config == 1 ? 4 : 8, 44100.0f);
    for (size_t i = 0; i < PITCH_SAMPLES; i += 256) {
        size_t count = PITCH_SAMPLES - i < 256 ? PITCH_SAMPLES - i : 256;
        smbPitchShift(count, q + i, out + i);
    }
    FILE *file = std::fopen(argv[2], "wb"); if (!file) return 5;
    for (int16_t sample : out) {
        if (std::abs(static_cast<int>(sample)) >= 32767) { std::fclose(file); return 6; }
        uint16_t bits = static_cast<uint16_t>(sample);
        if (std::fputc(bits & 255, file) == EOF || std::fputc(bits >> 8, file) == EOF) { std::fclose(file); return 7; }
    }
    return std::fclose(file) == 0 ? 0 : 8;
}
