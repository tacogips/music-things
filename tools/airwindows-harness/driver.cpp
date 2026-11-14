// SPDX-License-Identifier: MIT
#include AW_HEADER
#include "stimulus.h"
#include <fstream>
#include <iomanip>
#include <vector>
#include <sstream>
#include <utility>
#include <sys/utsname.h>
template<class T, class = void> struct stereo_fpd : std::false_type {};
template<class T> struct stereo_fpd<T, std::void_t<decltype(std::declval<T>().fpdL), decltype(std::declval<T>().fpdR)>> : std::true_type {};
template<class T, class = void> struct mono_fpd : std::false_type {};
template<class T> struct mono_fpd<T, std::void_t<decltype(std::declval<T>().fpd)>> : std::true_type {};
template<class T> void reseed(T& effect) { if constexpr (stereo_fpd<T>::value) { effect.fpdL = aw_seed(0x00c0ffee, 0); effect.fpdR = aw_seed(0x00c0ffee, 1); } else if constexpr (mono_fpd<T>::value) effect.fpd = aw_seed(0x00c0ffee, 0); }
static std::string escaped(const std::string& s) { std::string o; for (char c : s) { if (c == '\\') o += "\\\\"; else if (c == '\n') o += "\\n"; else o += c; } return o; }
int main(int argc, char** argv) {
 if (argc != 5) return 1;
 const float alt[] = {0.30f,0.85f,0.15f,0.65f,0.95f,0.45f,0.05f,0.70f,0.25f};
 std::ofstream meta(std::string(argv[1])+".txt"), data(std::string(argv[1])+".f32", std::ios::binary);
 if (!meta || !data) return 2;
 struct utsname host{}; uname(&host);
 meta << "format=aw-fixture-v1\nname=" << argv[2] << "\nupstream=" << argv[3] << "\nhost=" << host.machine << "-" << host.sysname << "\nclang=" << __clang_version__ << "\n";
 float l[AW_FRAMES],r[AW_FRAMES],ol[AW_FRAMES],orr[AW_FRAMES];
 auto checksum = [](float* a, float* b) { float interleaved[AW_FRAMES * 2]; for (int i=0;i<AW_FRAMES;i++) { interleaved[2*i]=a[i]; interleaved[2*i+1]=b[i]; } return f32_buffer_fnv(interleaved,AW_FRAMES*2); };
 aw_stimulus(44100,l,r); auto h441=checksum(l,r); aw_stimulus(96000,l,r); auto h960=checksum(l,r);
 meta << std::hex << std::setfill('0') << "stimulus_fnv_44100=" << std::setw(16) << h441 << "\nstimulus_fnv_96000=" << std::setw(16) << h960 << std::dec << "\nseed=00c0ffee\nblock=128\nframes=8192\ndescription=" << escaped(argv[4]) << "\nparams=" << kNumParameters << '\n';
 for(int render=0;render<3;render++) {
  float sr=render==2?96000.0f:44100.0f; aw_shim_initial_sample_rate=sr; aw_harness_srand(0x00c0ffee); AW_CLASS effect(nullptr); effect.setSampleRate(sr); reseed(effect);
  if (render==0) for(int i=0;i<kNumParameters;i++) { char n[256]={},label[256]={},display[256]={}; effect.getParameterName(i,n); effect.getParameterLabel(i,label); effect.getParameterDisplay(i,display); float v=effect.getParameter(i); uint32_t bits; std::memcpy(&bits,&v,4); meta << "param."<<i<<".name="<<n<<"\nparam."<<i<<".default="<<std::hex<<std::setw(8)<<bits<<std::dec<<"\nparam."<<i<<".label_r0="<<label<<"\nparam."<<i<<".display_r0="<<display<<'\n'; }
  if (render>0) for(int i=0;i<kNumParameters;i++) effect.setParameter(i,alt[i%9]);
  if (render==1) for(int i=0;i<kNumParameters;i++) { char label[256]={},display[256]={}; effect.getParameterLabel(i,label); effect.getParameterDisplay(i,display); meta << "param."<<i<<".label_r1="<<label<<"\nparam."<<i<<".display_r1="<<display<<'\n'; }
  aw_stimulus(sr,l,r);
  for(int frame=0;frame<AW_FRAMES;frame+=128) { float* in[]={l+frame,r+frame}; float* out[]={ol+frame,orr+frame}; effect.processReplacing(in,out,128); }
  for(int k=0;k<504;k++) { int n=k<256?k:256+32*(k-256); data.write(reinterpret_cast<char*>(&ol[n]),4); data.write(reinterpret_cast<char*>(&orr[n]),4); }
 }
 return meta && data ? 0 : 3;
}
