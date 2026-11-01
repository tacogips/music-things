# Third-Party Notices

music-things is licensed under the MIT License (see `LICENSE`). Code ported
from third-party projects keeps its original license; each ported component is
listed below with its upstream source, pinned revision and license text.

The reference harnesses in `tools/` compile upstream code in place from `MUSIC_THINGS_REFS`; they do not redistribute it.

## Airwindows

Upstream: https://github.com/airwindows/airwindows

Pinned revision/version: `b110fac3991fff7c4655cc2d25d6a3e221f9799c`.

Ported files: `plugins/LinuxVST/src/<Name>/{<Name>.h,<Name>.cpp,<Name>Proc.cpp} → crates/airwindows/src/plugins/batch1/`.

Batch 1 names in order:

`Acceleration`, `Acceleration2`, `ADClip7`, `ADT`, `Air`, `Air2`, `Apicolypse`, `AQuickVoiceClip`, `AtmosphereBuss`, `AtmosphereChannel`, `Aura`, `AutoPan`, `Average`, `AverMatrix`, `Balanced`, `BassAmp`, `BassDrive`, `BassKit`, `Baxandall`, `Baxandall2`, `Beam`, `BigAmp`, `Biquad`, `Biquad2`, `BiquadDouble`, `BiquadOneHalf`, `BiquadPlus`, `BiquadTriple`, `Bite`, `BitGlitter`, `BitShiftGain`, `BlockParty`, `BrassRider`, `BrightAmbience`, `BrightAmbience2`, `BrightAmbience3`, `BuildATPDF`, `BussColors4`, `ButterComp`, `ButterComp2`, `C5RawBuss`, `C5RawChannel`, `Cabs`, `Calibre`, `Capacitor`, `Capacitor2`, `Chamber`, `Chamber2`, `Channel4`, `Channel5`, `Channel6`, `Channel7`, `Channel8`, `Channel9`, `Chorus`, `ChorusEnsemble`, `ChromeOxide`, `Cider`, `ClipOnly`, `ClipOnly2`, `ClipSoftly`, `Coils`, `Coils2`, `Cojones`, `Compresaturator`, `Console4Buss`, `Console4Channel`, `Console5Buss`, `Console5Channel`, `Console5DarkCh`, `Console6Buss`, `Console6Channel`, `Console7Buss`, `Console7Cascade`, `Console7Channel`, `Console7Crunch`, `Console8BusOut`, `Console8BussHype`, `Console8BussIn`, `Console8ChannelHype`, `Console8ChannelIn`, `Console8ChannelOut`, `Console8SubHype`, `Console8SubIn`, `Console8SubOut`, `CrunchyGrooveWear`, `Crystal`, `CStrip`, `curve`, `Dark`, `DarkNoise`, `DCVoltage`, `DeBess`, `Deckwrecka`, `DeEss`, `DeHiss`, `Density`, `Density2`, `DeRez`, `DeRez2`, `Desk`, `Desk4`, `DigitalBlack`, `Dirt`, `Distance`, `Distance2`, `Distortion`, `Ditherbox`, `DitherFloat`, `DitherMeDiskers`

```text
MIT License

Copyright (c) 2018 Chris Johnson

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## TinySoundFont tsf.h

Upstream: https://github.com/schellingb/TinySoundFont

Pinned revision/version: `853a0a171759f1ddba0de1442133a75912bbeffa`.

Ported files: `tsf.h → crates/soundfont/src/font/ and crates/soundfont/src/synth/`.

```text
Copyright (C) 2017-2023 Bernhard Schelling (Based on SFZero, Copyright (C) 2012 Steve Folta, https://github.com/stevefolta/SFZero)

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## TinyMidiLoader tml.h

Upstream: https://github.com/schellingb/TinySoundFont

Pinned revision/version: `853a0a171759f1ddba0de1442133a75912bbeffa`.

Ported files: `tml.h → crates/soundfont/src/midi/`.

```text
Copyright (C) 2017-2023 Bernhard Schelling (Based on SFZero, Copyright (C) 2012 Steve Folta, https://github.com/stevefolta/SFZero)

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## stftPitchShift

Upstream: https://github.com/jurihock/stftPitchShift

Pinned revision/version: `70c96db087458f8631edbc1bb4068d42ea5b52cc`.

Ported files: `cpp/StftPitchShift/ → crates/pitch/src/stft/`.

```text
MIT License

Copyright (c) 2022 Juergen Hock

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## dywapitchtrack

Upstream: https://hackage.haskell.org/package/dywapitchtrack-0.1.0.1

Pinned revision/version: `0.1.0.1`.

Ported files: `cbits/dywapitchtrack.c and include/dywapitchtrack.h → crates/pitch/src/dywa.rs`.

```text
/* dywapitchtrack.c
 
 Dynamic Wavelet Algorithm Pitch Tracking library
 Released under the MIT open source licence
  
 Copyright (c) 2010 Antoine Schmitt
 
 Permission is hereby granted, free of charge, to any person obtaining a copy
 of this software and associated documentation files (the "Software"), to deal
 in the Software without restriction, including without limitation the rights
 to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 copies of the Software, and to permit persons to whom the Software is
 furnished to do so, subject to the following conditions:
 
 The above copyright notice and this permission notice shall be included in
 all copies or substantial portions of the Software.
 
 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 THE SOFTWARE.
*/
```

## smbPitchShift

Upstream: https://github.com/s60sc/ESP32_VoiceChanger

Pinned revision/version: `1.2`.

Ported files: `smbPitchShift.esp32.cpp → crates/pitch/src/smb.rs`.

```text
/****************************************************************************
*
* NAME: smbPitchShift.cpp
* VERSION: 1.2
* HOME URL: http://www.dspdimension.com
* KNOWN BUGS: none
*
* SYNOPSIS: Routine for doing pitch shifting while maintaining
* duration using the Short Time Fourier Transform.
*
* DESCRIPTION: The routine takes a pitchShift factor value which is between 0.5
* (one octave down) and 2. (one octave up). A value of exactly 1 does not change
* the pitch. numSampsToProcess tells the routine how many samples in indata[0...
* numSampsToProcess-1] should be pitch shifted and moved to outdata[0 ...
* numSampsToProcess-1]. The two buffers can be identical (ie. it can process the
* data in-place). fftFrameSize defines the FFT frame size used for the
* processing. Typical values are 1024, 2048 and 4096. It may be any value <=
* MAX_FRAME_LENGTH but it MUST be a power of 2. osamp is the STFT
* oversampling factor which also determines the overlap between adjacent STFT
* frames. It should at least be 4 for moderate scaling ratios. A value of 32 is
* recommended for best quality. sampleRate takes the sample rate for the signal 
* in unit Hz, ie. 44100 for 44.1 kHz audio. The data passed to the routine in 
* indata[] should be in the range [-1.0, 1.0), which is also the output range 
* for the data, make sure you scale the data accordingly (for 16bit signed integers
* you would have to divide (and multiply) by 32768). 
*
* COPYRIGHT 1999-2009 Stephan M. Bernsee <smb [AT] dspdimension [DOT] com>
*
* 						The Wide Open License (WOL)
*
* Permission to use, copy, modify, distribute and sell this software and its
* documentation for any purpose is hereby granted without fee, provided that
* the above copyright notice and this license appear in all source copies. 
* THIS SOFTWARE IS PROVIDED "AS IS" WITHOUT EXPRESS OR IMPLIED WARRANTY OF
* ANY KIND. See http://www.dspguru.com/wol.htm for more information.
*
*****************************************************************************/
```
