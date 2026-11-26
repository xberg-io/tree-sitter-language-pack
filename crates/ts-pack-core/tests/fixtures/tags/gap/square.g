Square := function(x)
  return x * x;
end;

Double := x -> 2 * x;

Cube := function(x)
  local sq;
  sq := Square(x);
  return sq * x;
end;

Counter := atomic function(n)
  return n + 1;
end;

Print(Square(3), Cube(2), Counter(1));
