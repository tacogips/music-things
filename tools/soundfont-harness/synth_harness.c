/* MIT License; Copyright (c) 2026 music-things contributors.
 * Permission is granted to use, copy, modify, publish and distribute this
 * software with this notice. THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY. */
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#define TSF_IMPLEMENTATION
#include "tsf.h"
#define TML_IMPLEMENTATION
#include "tml.h"

static int write_float(FILE *out, float value) {
    uint32_t bits;
    unsigned char bytes[4];
    memcpy(&bits, &value, sizeof bits);
    for (int i = 0; i < 4; ++i) bytes[i] = (unsigned char)(bits >> (8 * i));
    return fwrite(bytes, 1, sizeof bytes, out) == sizeof bytes;
}
static int store(FILE *out, const float *buffer, int channels, int block) {
    for (int i = 0; i < 64; ++i) {
        int frame = block * 64 + i;
        if (frame >= 2048 && frame % 16) continue;
        for (int c = 0; c < channels; ++c)
            if (!write_float(out, buffer[i * channels + c])) return 0;
    }
    return 1;
}
static void dispatch(tsf *s, const tml_message *m) {
    switch (m->type) {
        case TML_PROGRAM_CHANGE: tsf_channel_set_presetnumber(s, m->channel, m->program, m->channel == 9); break;
        case TML_NOTE_ON: tsf_channel_note_on(s, m->channel, m->key, m->velocity / 127.0f); break;
        case TML_NOTE_OFF: tsf_channel_note_off(s, m->channel, m->key); break;
        case TML_PITCH_BEND: tsf_channel_set_pitchwheel(s, m->channel, m->pitch_bend); break;
        case TML_CONTROL_CHANGE: tsf_channel_midi_control(s, m->channel, m->control, m->control_value); break;
    }
}
static int scenario1(const char *font, const char *midi, const char *path) {
    tsf *s = tsf_load_filename(font);
    tml_message *head = tml_load_filename(midi), *message = head;
    FILE *out = fopen(path, "wb");
    if (!s || !head || !out) return 0;
    tsf_set_output(s, TSF_STEREO_INTERLEAVED, 44100, 0.0f);
    if (!tsf_set_max_voices(s, 32)) return 0;
    tsf_channel_set_bank_preset(s, 9, 128, 0);
    double msec = 0;
    for (int k = 0; k < 1378; ++k) {
        float buffer[128];
        msec += 64.0 * (1000.0 / 44100.0);
        while (message && message->time <= msec) { dispatch(s, message); message = message->next; }
        tsf_render_float(s, buffer, 64, 0);
        if (!store(out, buffer, 2, k)) return 0;
    }
    tml_free(head); tsf_close(s);
    return fclose(out) == 0;
}
static int scenario2(const char *font, const char *path) {
    tsf *s = tsf_load_filename(font);
    FILE *out = fopen(path, "wb");
    if (!s || !out) return 0;
    tsf_set_output(s, TSF_MONO, 44100, -6.0f);
    if (!tsf_set_max_voices(s, 8)) return 0;
    for (int k = 0; k < 1378; ++k) {
        float buffer[64];
        if (k == 0) {
            tsf_channel_set_presetnumber(s, 0, 0, 0);
            for (int key = 48; key <= 60; ++key) tsf_channel_note_on(s, 0, key, 0.8f);
        }
        if (k == 100) { tsf_channel_set_pitchrange(s, 0, 12.0f); tsf_channel_set_pitchwheel(s, 0, 12000); }
        if (k == 200) tsf_channel_midi_control(s, 0, 64, 127);
        if (k == 220) for (int key = 48; key <= 60; ++key) tsf_channel_note_off(s, 0, key);
        if (k == 400) tsf_channel_midi_control(s, 0, 64, 0);
        if (k == 500) tsf_note_on(s, 1, 72, 1.0f);
        if (k == 600) { tsf_channel_set_pan(s, 0, 0.2f); tsf_channel_set_volume(s, 0, 0.5f); }
        if (k == 800) tsf_note_off_all(s);
        if (k == 900) tsf_channel_note_on(s, 0, 60, 0.5f);
        if (k == 1000) tsf_channel_sounds_off_all(s, 0);
        for (int i = 0; i < 64; ++i) buffer[i] = 0.125f;
        tsf_render_float(s, buffer, 64, 1);
        if (!store(out, buffer, 1, k)) return 0;
    }
    tsf_close(s);
    return fclose(out) == 0;
}
int main(int argc, char **argv) {
    if (argc != 5) { fprintf(stderr, "usage: %s SF2 MIDI S1 S2\n", argv[0]); return 2; }
    if (!scenario1(argv[1], argv[2], argv[3]) || !scenario2(argv[1], argv[4])) {
        fprintf(stderr, "synth harness failed\n"); return 1;
    }
    return 0;
}
