// SPDX-License-Identifier: MIT
// Written from scratch; contains no Steinberg SDK code.
#include "audioeffectx.h"
#include <cmath>
#include <string>
float aw_shim_initial_sample_rate = 44100.0f;
static std::string convert(float v) {
    double x = v;
    if (std::isnan(x)) return "nan";
    if (std::fabs(x) >= 1e8) return "Huge!";
    bool neg = x < 0; double a = std::fabs(x), whole = std::floor(a);
    std::string result = std::to_string(static_cast<uint32_t>(whole));
    int digits = static_cast<int>(result.size());
    if (digits < 8) { result += '.'; double f = a - whole; for (int i = digits; i < 8; ++i) { f *= 10; int d = static_cast<int>(std::floor(f)); result += static_cast<char>('0' + d); f -= d; } }
    return neg ? "-" + result : result;
}
void float2string(float v, char* s, VstInt32 n) { vst_strncpy(s, convert(v).c_str(), n); }
void dB2string(float v, char* s, VstInt32 n) { vst_strncpy(s, (v <= 0 ? std::string("-oo") : convert(static_cast<float>(20.0 * std::log10(static_cast<double>(v))))).c_str(), n); }
void int2string(VstInt32 v, char* s, VstInt32 n) { vst_strncpy(s, (v >= 100000000 || v <= -10000000 ? std::string("Huge!") : std::to_string(v)).c_str(), n); }
