SinOsc s => dac;

440.0 => float base;

class Voice extends Object
{
    0.5 => float gain;

    fun void play(float freq)
    {
        freq => s.freq;
        gain => s.gain;
    }
}

fun float double(float x)
{
    x * 2.0 => float result;
    return result;
}

Voice v;
v.play(double(base));
1::second => now;
