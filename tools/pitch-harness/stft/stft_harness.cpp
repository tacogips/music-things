// SPDX-License-Identifier: MIT
// Reference harness for stftPitchShift, revision 70c96db087458f8631edbc1bb4068d42ea5b52cc.
#include <StftPitchShift/StftPitchShift.h>
#include "../../common/stimulus.h"
#include <bit>
#include <cstdio>
#include <cstdint>
#include <filesystem>
#include <stdexcept>
#include <tuple>
#include <vector>

static void write_fixture(const std::filesystem::path& path, const std::vector<float>& data) {
  FILE* file = std::fopen(path.string().c_str(), "wb");
  if (!file) throw std::runtime_error("cannot open fixture");
  for (float value : data) {
    uint32_t bits = std::bit_cast<uint32_t>(value);
    unsigned char bytes[4] = {static_cast<unsigned char>(bits), static_cast<unsigned char>(bits >> 8), static_cast<unsigned char>(bits >> 16), static_cast<unsigned char>(bits >> 24)};
    if (std::fwrite(bytes, 1, sizeof bytes, file) != sizeof bytes) throw std::runtime_error("cannot write fixture");
  }
  if (std::fclose(file)) throw std::runtime_error("cannot close fixture");
}

int main(int argc, char** argv) {
  if (argc != 2) return 2;
  std::vector<float> input(PITCH_SAMPLES), output(PITCH_SAMPLES);
  pitch_stimulus(input.data());
  std::printf("pitch_fnv=%016llx\n", static_cast<unsigned long long>(f32_buffer_fnv(input.data(), input.size())));
  std::filesystem::path dir(argv[1]);
  stftpitchshift::StftPitchShift c1(1024, 256, 44100, false);
  c1.shiftpitch(input, output, std::vector<double>{1.5}, 0.001, 1);
  write_fixture(dir / "c1.f32", output);
  stftpitchshift::StftPitchShift c2(1024, 256, 44100, true);
  c2.shiftpitch(input, output, std::vector<double>{0.75, 1.25}, 0, 1);
  write_fixture(dir / "c2.f32", output);
  stftpitchshift::StftPitchShift c3(std::make_tuple(size_t(1024), size_t(512)), 128, 44100, false);
  c3.shiftpitch(input, output, std::vector<double>{1.25}, 0, 1);
  write_fixture(dir / "c3.f32", output);
}
