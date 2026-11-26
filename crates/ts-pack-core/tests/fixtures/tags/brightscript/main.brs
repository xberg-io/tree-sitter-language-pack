sub Main()
    total = Add(1, 2)
    print FormatTotal(total)
end sub

function Add(a as integer, b as integer) as integer
    return a + b
end function

function FormatTotal(value as integer) as string
    return "Total: " + value.ToStr()
end function
