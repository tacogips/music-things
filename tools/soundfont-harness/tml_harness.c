/*
 * MIT License
 * 
 * Copyright (c) 2026 music-things contributors
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
 * of the Software, and to permit persons to whom the Software is furnished to do
 * so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#define TML_IMPLEMENTATION
#include "tml.h"

int main(int argc, char **argv)
{
    FILE *file;
    long length;
    unsigned char *data;
    tml_message *first, *msg;
    int channels, programs, notes;
    unsigned int first_note, duration;
    if (argc != 2 || !(file = fopen(argv[1], "rb"))) return 1;
    if (fseek(file, 0, SEEK_END) || (length = ftell(file)) < 0 || length > INT_MAX ||
        fseek(file, 0, SEEK_SET)) { fclose(file); return 1; }
    data = malloc(length ? (size_t)length : 1);
    if (!data || fread(data, 1, (size_t)length, file) != (size_t)length) {
        free(data); fclose(file); return 1;
    }
    if (fclose(file)) { free(data); return 1; }
    first = tml_load_memory(data, (int)length);
    free(data);
    if (!first) { puts("error"); return ferror(stdout) ? 1 : 0; }
    for (msg = first; msg; msg = msg->next) {
        unsigned int a = 0, b = 0;
        switch (msg->type) {
        case TML_NOTE_ON: case TML_NOTE_OFF:
            a = (unsigned char)msg->key; b = (unsigned char)msg->velocity; break;
        case TML_KEY_PRESSURE:
            a = (unsigned char)msg->key; b = (unsigned char)msg->key_pressure; break;
        case TML_CONTROL_CHANGE:
            a = (unsigned char)msg->control; b = (unsigned char)msg->control_value; break;
        case TML_PROGRAM_CHANGE: a = (unsigned char)msg->program; break;
        case TML_CHANNEL_PRESSURE: a = (unsigned char)msg->channel_pressure; break;
        case TML_PITCH_BEND: a = msg->pitch_bend; break;
        case TML_SET_TEMPO: a = (unsigned int)tml_get_tempo_value(msg); break;
        }
        printf("%u %02x %u %u %u\n", msg->time, msg->type, msg->channel, a, b);
    }
    tml_get_info(first, &channels, &programs, &notes, &first_note, &duration);
    printf("info %d %d %d %u %u\n", channels, programs, notes, first_note, duration);
    tml_free(first);
    return ferror(stdout) ? 1 : 0;
}
