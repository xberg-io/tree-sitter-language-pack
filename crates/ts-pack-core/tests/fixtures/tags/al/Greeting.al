namespace Contoso.Greeting;

using Microsoft.Sales.Customer;

enum 50100 GreetingStyle implements IGreeter
{
    Extensible = true;

    value(0; Formal)
    {
        Caption = 'Formal';
        Implementation = IGreeter = Greeter;
    }
    value(1; Casual) { Caption = 'Casual'; }
}

enumextension 50101 GreetingStyleExt extends GreetingStyle
{
    value(50100; Pirate) { Caption = 'Pirate'; }
}

interface IGreeter
{
    procedure Greet(Name: Text): Text;
}

interface IFancyGreeter extends IGreeter
{
    procedure Flourish(): Text;
}

table 50100 GreetingEntry
{
    fields
    {
        field(1; EntryNo; Integer) { }
        field(2; Message; Text[100])
        {
            trigger OnValidate()
            begin
                Message := DelChr(Message, '<>');
            end;
        }
        field(3; Style; Enum GreetingStyle) { }
    }
    keys
    {
        key(PK; EntryNo) { Clustered = true; }
    }
    fieldgroups
    {
        fieldgroup(DropDown; EntryNo, Message) { }
    }

    trigger OnInsert()
    begin
        Style := GreetingStyle::Formal;
    end;
}

tableextension 50102 CustomerGreeting extends Customer
{
    fields
    {
        field(50100; GreetingText; Text[100]) { }
    }
}

codeunit 50101 Greeter implements IGreeter
{
    var
        GreetCount: Integer;

    trigger OnRun()
    begin
        Greet('World');
    end;

    procedure Greet(Name: Text): Text
    var
        Entry: Record GreetingEntry;
        Result: Text;
    begin
        GreetCount += 1;
        Entry.SetRange(EntryNo, Database::GreetingEntry);
        Result := StrSubstNo('Hello %1', Name);
        Entry.Message := Result;
        Entry.Insert();
        LogGreeting;
        exit(Result);
    end;

    local procedure LogGreeting()
    begin
    end;

    [IntegrationEvent(false, false)]
    local procedure OnAfterGreet(Name: Text)
    begin
    end;
}

page 50100 GreetingList
{
    PageType = List;
    SourceTable = GreetingEntry;

    layout
    {
        area(Content)
        {
            repeater(Lines)
            {
                field(Message; Rec.Message) { }
            }
        }
    }

    actions
    {
        area(Processing)
        {
            action(SayHello)
            {
                trigger OnAction()
                var
                    GreeterCU: Codeunit Greeter;
                begin
                    Message(GreeterCU.Greet('Page'));
                end;
            }
        }
    }

    views
    {
        view(CasualOnly)
        {
            Filters = where(Style = const(Casual));
        }
    }
}

pageextension 50103 CustomerCardGreeting extends "Customer Card"
{
}

report 50104 GreetingReport
{
    dataset
    {
        dataitem(Entry; GreetingEntry) { }
    }

    labels
    {
        TitleLbl = 'Greetings';
    }
}

query 50105 GreetingQuery
{
    elements
    {
        dataitem(Greeting; GreetingEntry)
        {
            column(Message; Message) { }
        }
    }
}

xmlport 50106 GreetingExport
{
    schema
    {
        textelement(Root)
        {
        }
    }
}

controladdin GreetingBanner
{
    Scripts = 'banner.js';

    event BannerClicked(Name: Text);

    procedure Show(Text: Text);
}

permissionset 50107 GreetingAdmin
{
    Permissions = tabledata GreetingEntry = RIMD;
}

permissionsetextension 50108 GreetingAdminExt extends "D365 BASIC"
{
}

profile GreetingProfile
{
    RoleCenter = GreetingList;
}

entitlement GreetingEntitlement
{
    Type = Role;
}
