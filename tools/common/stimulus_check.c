/* SPDX-License-Identifier: MIT */
#include "stimulus.h"
#include <inttypes.h>
#include <stdio.h>
static uint64_t aw_hash(float rate) {
    float l[AW_FRAMES], r[AW_FRAMES], interleaved[2 * AW_FRAMES];
    aw_stimulus(rate, l, r);
    for (size_t i = 0; i < AW_FRAMES; ++i) { interleaved[2*i] = l[i]; interleaved[2*i+1] = r[i]; }
    return f32_buffer_fnv(interleaved, 2 * AW_FRAMES);
}
int main(void) {
    float pitch[PITCH_SAMPLES]; pitch_stimulus(pitch);
    printf("aw44100=%016" PRIx64 "\n", aw_hash(44100.0f));
    printf("aw96000=%016" PRIx64 "\n", aw_hash(96000.0f));
    printf("pitch=%016" PRIx64 "\n", f32_buffer_fnv(pitch, PITCH_SAMPLES));
    return 0;
}
