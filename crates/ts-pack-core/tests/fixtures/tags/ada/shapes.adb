with Ada.Text_IO;

package body Shapes is

   Max_Sides : constant Integer := 8;
   Count : Integer := 0;

   type Color is (Red, Green, Blue);

   type Point is record
      X : Integer;
      Y : Integer;
   end record;

   subtype Small is Integer range 0 .. 10;

   Overflow : exception;

   function Area (Width, Height : Integer) return Integer is
      Result : Integer := 0;
   begin
      Result := Width * Height;
      return Result;
   end Area;

   procedure Report (P : Point) is
   begin
      Ada.Text_IO.Put_Line (Integer'Image (Area (P.X, P.Y)));
   end Report;

end Shapes;
