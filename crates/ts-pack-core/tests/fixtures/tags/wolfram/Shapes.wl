BeginPackage["Shapes`"]

CircleArea::usage = "CircleArea[r] gives the area of a circle of radius r."

Begin["`Private`"]

$PiApprox = 3.14

square[x_] := x^2

cube[x_] = x^3

defaultRadius := RandomReal[]

CircleArea[r_?NumericQ] := $PiApprox * square[r]

TotalArea[radii_List] := Total[CircleArea /@ radii]

End[]

EndPackage[]
