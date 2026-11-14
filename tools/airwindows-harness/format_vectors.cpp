// SPDX-License-Identifier: MIT
#include "audioeffectx.h"
#include <cstdio>
#include <cstring>
#include <limits>
int main() {
 const float values[]={0,0.5f,1,-1,1e-7f,0.123456789f,12.25f,99999999.f,1e8f,-1e7f,std::numeric_limits<float>::quiet_NaN(),1e-6f,2,-3,0.1f,0.3f,0.7f,42.4f,99999.5f};
 char s[64]; for(float v:values) { uint32_t bits; std::memcpy(&bits,&v,4); float2string(v,s,64); std::printf("f2s %08x %s\n",bits,s); }
 for(float v:values) { uint32_t bits; std::memcpy(&bits,&v,4); dB2string(v,s,64); std::printf("db %08x %s\n",bits,s); }
 const int ints[]={0,1,-1,99999999,100000000,-9999999,-10000000,42}; for(int v:ints) { int2string(v,s,64); std::printf("i2s %d %s\n",v,s); }
}
