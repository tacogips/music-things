/* MIT License; Copyright (c) 2026 music-things contributors.
 * Permission is granted, free of charge, to use, copy, modify, merge, publish,
 * distribute, sublicense and/or sell this software, subject to inclusion of
 * this notice. THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND. */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#define TSF_IMPLEMENTATION
#include "tsf.h"
static void f(float v) { uint32_t n; memcpy(&n, &v, 4); printf(" %08x", n); }
static void env(struct tsf_envelope *e) {
    f(e->delay); f(e->attack); f(e->hold); f(e->decay); f(e->sustain);
    f(e->release); f(e->keynumToHold); f(e->keynumToDecay);
}
int main(int argc, char **argv) {
    if (argc != 2) return 2;
    FILE *fp = fopen(argv[1], "rb"); if (!fp) return 2;
    if (fseek(fp, 0, SEEK_END)) return 2;
    long length = ftell(fp); if (length < 0 || fseek(fp, 0, SEEK_SET)) return 2;
    unsigned char *data = malloc((size_t)length); if (!data) return 2;
    if (fread(data, 1, (size_t)length, fp) != (size_t)length) return 2;
    fclose(fp);
    tsf *font = tsf_load_memory(data, (int)length);
    if (!font) { puts("error"); free(data); return 0; }
    unsigned int count = 0;
    for (long i = 0; i + 8 <= length; ++i) if (!memcmp(data+i, "smpl", 4)) {
        count = ((unsigned)data[i+4] | (unsigned)data[i+5]<<8 | (unsigned)data[i+6]<<16 | (unsigned)data[i+7]<<24) / 2; break;
    }
    if (!count) return 2;
    printf("presets %d\n", font->presetNum);
    for (int i = 0; i < font->presetNum; ++i) {
        struct tsf_preset *p = &font->presets[i];
        printf("preset %d %u %u %s %d\n", i, p->bank, p->preset, p->presetName, p->regionNum);
        for (int j = 0; j < p->regionNum; ++j) {
            struct tsf_region *r = &p->regions[j];
            printf("region %d %u %u %u %u %u %u %u %u %u %u %d %d %d %d", r->loop_mode, r->sample_rate, r->lokey, r->hikey, r->lovel, r->hivel, r->group, r->offset, r->end, r->loop_start, r->loop_end, r->transpose, r->tune, r->pitch_keycenter, r->pitch_keytrack);
            f(r->attenuation); f(r->pan); env(&r->ampenv); env(&r->modenv);
            printf(" %d %d %d %d %d %d", r->initialFilterQ, r->initialFilterFc, r->modEnvToPitch, r->modEnvToFilterFc, r->modLfoToFilterFc, r->modLfoToVolume);
            f(r->delayModLFO); printf(" %d %d", r->freqModLFO, r->modLfoToPitch);
            f(r->delayVibLFO); printf(" %d %d\n", r->freqVibLFO, r->vibLfoToPitch);
        }
    }
    uint64_t hash = UINT64_C(0xcbf29ce484222325);
    for (unsigned i = 0; i < count; ++i) {
        uint32_t bits; memcpy(&bits, &font->fontSamples[i], 4);
        for (int b = 0; b < 4; ++b) { hash ^= (bits >> (8*b)) & 255; hash *= UINT64_C(0x100000001b3); }
    }
    printf("samples %u %016llx\n", count, (unsigned long long)hash);
    tsf_close(font); free(data); return 0;
}
