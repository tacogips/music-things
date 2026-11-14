/* SPDX-License-Identifier: MIT */
/* Reference driver for dywapitchtrack 0.1.0.1; upstream is compiled in place. */
#include <inttypes.h>
#include <stdio.h>
#include "dywapitchtrack.h"
#include "stimulus.h"

int main(int argc, char **argv) {
    float input[PITCH_SAMPLES];
    double samples[PITCH_SAMPLES];
    const int windows[] = {1024, 2048};
    if (argc != 3) {
        fprintf(stderr, "usage: %s pitches.txt meta.txt\n", argv[0]);
        return 1;
    }
    FILE *pitches = fopen(argv[1], "w");
    FILE *meta = fopen(argv[2], "w");
    if (!pitches || !meta) {
        perror("fixture open");
        if (pitches) fclose(pitches);
        if (meta) fclose(meta);
        return 1;
    }
    pitch_stimulus(input);
    for (int i = 0; i < PITCH_SAMPLES; ++i) samples[i] = (double)input[i];
    fprintf(meta, "pitch=%016" PRIx64 "\n", f32_buffer_fnv(input, PITCH_SAMPLES));
    for (size_t i = 0; i < sizeof windows / sizeof windows[0]; ++i) {
        int w = windows[i];
        dywapitchtracker tracker;
        dywapitch_inittracking(&tracker);
        for (int start = 0; start + w <= PITCH_SAMPLES; start += w) {
            double pitch = dywapitch_computepitch(&tracker, samples, start, w);
            fprintf(pitches, "%d %d %.17g\n", w, start, pitch);
        }
    }
    int a = fclose(pitches);
    int b = fclose(meta);
    return a != 0 || b != 0;
}
