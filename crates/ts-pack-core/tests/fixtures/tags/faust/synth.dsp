import("stdfaust.lib");

declare name "synth";

freq = hslider("freq", 440, 50, 2000, 1);
gain = 0.5;

voice(f) = os.osc(f) * gain;

filters = environment {
  cutoff = 1000;
  lowpass(c) = fi.lowpass(2, c);
};

process = voice(freq) : filters.lowpass(1000) with {
  local = 1;
};
