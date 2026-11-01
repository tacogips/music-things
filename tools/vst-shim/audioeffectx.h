// SPDX-License-Identifier: MIT
// Written from scratch; contains no Steinberg SDK code.
#ifndef __audioeffect__
#define __audioeffect__
#include <cstdint>
#include <cstring>
using VstInt32 = int32_t;
using VstIntPtr = intptr_t;
using audioMasterCallback = VstIntPtr (*)(void*, VstInt32, VstInt32, VstIntPtr, void*, float);
enum VstPlugCategory { kPlugCategEffect = 1 };
constexpr int kVstMaxParamStrLen = 64, kVstMaxProgNameLen = 64, kVstMaxProductStrLen = 64, kVstMaxVendorStrLen = 64;
extern float aw_shim_initial_sample_rate;
class AudioEffect {
    float sampleRate;
public:
    AudioEffect(audioMasterCallback, VstInt32, VstInt32): sampleRate(aw_shim_initial_sample_rate) {}
    virtual ~AudioEffect() = default;
    virtual float getParameter(VstInt32) { return 0.0f; }
    virtual void setParameter(VstInt32, float) {}
    virtual void getParameterName(VstInt32, char*) {}
    virtual void getParameterLabel(VstInt32, char*) {}
    virtual void getParameterDisplay(VstInt32, char*) {}
    float getSampleRate() const { return sampleRate; }
    void setSampleRate(float value) { sampleRate = value; }
    void setNumInputs(int) {} void setNumOutputs(int) {} void setUniqueID(unsigned long) {}
    void canProcessReplacing() {} void canDoubleReplacing() {} void programsAreChunks(bool) {}
};
using AudioEffectX = AudioEffect;
inline void vst_strncpy(char* dst, const char* src, int n) { std::strncpy(dst, src, n); dst[n-1] = 0; }
inline void vst_strncat(char* dst, const char* src, int n) { size_t len = std::strlen(dst); if (len < static_cast<size_t>(n - 1)) std::strncat(dst, src, n - 1 - len); }
void float2string(float, char*, VstInt32);
void dB2string(float, char*, VstInt32);
void int2string(VstInt32, char*, VstInt32);
#endif
