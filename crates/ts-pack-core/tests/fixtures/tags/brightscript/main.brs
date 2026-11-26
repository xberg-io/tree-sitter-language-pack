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

function Average(values as object) as float
    if values.Count() = 0 then return 0
    return Add(values[0], values[1]) / values.Count()
end function
