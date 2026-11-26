codeunit 50100 Greeter
{
    procedure Greet(Name: Text): Text
    begin
        exit(Format(Name));
    end;

    local procedure Helper()
    begin
    end;
}
