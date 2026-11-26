set title "Damped oscillation"
amplitude = 2.5
decay = 0.3
array offsets[2] = [0.5, 1.5]
f(x) = amplitude * exp(-decay * x) * cos(x)
g(x, y) = f(x) + y

$samples << EOD
0 1
1 2
EOD

plot f(x) title "signal", $samples using 1:2 with points
