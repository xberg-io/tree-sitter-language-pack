with Ada.Text_IO;

package Geometry is

   Pi : constant := 3.14159;
   Origin_X : constant Integer := 0;
   Scale : Float := 1.0;

   type Kind is (Circle, Square);

   type Vector is record
      X : Float;
      Y : Float;
   end record;

   type Drawable is interface;

   type Handle is private;

   subtype Percent is Integer range 0 .. 100;

   function Length (V : Vector) return Float;

   function Twice (X : Float) return Float is (2.0 * X);

   procedure Reset is null;

   generic
      type Element is private;
   procedure Swap (A, B : in out Element);

private

   type Handle is new Integer;

end Geometry;

package body Shapes is

   Max_Sides : constant Integer := 8;
   Count : Integer := 0;

   type Color is (Red, Green, Blue);

   type Point is record
      X : Integer;
      Y : Integer;
   end record;

   type Shape is interface;

   subtype Small is Integer range 0 .. 10;

   Overflow : exception;

   function Perimeter (Sides : Integer) return Integer;

   function Area (Width, Height : Integer) return Integer is
      Result : Integer := 0;
   begin
      Result := Width * Height;
      return Result;
   end Area;

   function Perimeter (Sides : Integer) return Integer is
   begin
      return Sides * 4;
   end Perimeter;

   procedure Report (P : Point) is
   begin
      Ada.Text_IO.Put_Line (Integer'Image (Area (P.X, P.Y)));
   end Report;

end Shapes;

procedure Main is
begin
   Shapes.Report ((1, 2));
end Main;
