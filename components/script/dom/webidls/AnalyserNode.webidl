/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/*
 * The origin of this IDL file is
 * https://webaudio.github.io/web-audio-api/#analysernode
 */

dictionary AnalyserOptions : AudioNodeOptions {
  unsigned long fftSize = 2048;
  double maxDecibels = -30;
  double minDecibels = -100;
  double smoothingTimeConstant = 0.8;
};

[Exposed=Window,
 Constructor (BaseAudioContext context, optional AnalyserOptions options)]
interface AnalyserNode : AudioNode {
  void getFloatFrequencyData (Float32Array array);
  void getByteFrequencyData (Uint8Array array);
  void getFloatTimeDomainData (Float32Array array);
  void getByteTimeDomainData (Uint8Array array);
  [SetterThrows] attribute unsigned long fftSize;
  readonly attribute unsigned long frequencyBinCount;
  [SetterThrows] attribute double minDecibels;
  [SetterThrows] attribute double maxDecibels;
  [SetterThrows] attribute double smoothingTimeConstant;
};
