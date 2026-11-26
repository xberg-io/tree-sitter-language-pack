program Shapes;

uses SysUtils;

const
  MaxSides = 8;

type
  TColor = (Red, Green, Blue);

  TLength = Double;

  TPoint = record
    X, Y: Integer;
  end;

  IShape = interface
    function Perimeter: Double;
  end;

  TShape = class
  private
    FSides: Integer;
  public
    function Area: Double; virtual;
  end;

var
  Counter: Integer;

function TShape.Area: Double;
begin
  Result := 0.0;
end;

function Add(A, B: Integer): Integer;
var
  Sum: Integer;
begin
  Sum := A + B;
  Add := Sum;
end;

procedure Report(N: Integer);
type
  TPair = record
    A, B: Integer;
  end;
begin
  WriteLn(IntToStr(N));
end;

begin
  Counter := Add(1, 2);
  Report(Counter);
end.
