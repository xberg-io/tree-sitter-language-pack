namespace eval counter {
    variable count 0
    namespace export increment
}

set limit 10

proc counter::increment {step} {
    variable count
    set count [clamp [expr {$count + $step}]]
    return $count
}

proc clamp {value} {
    global limit
    if {$value > $limit} {
        return $limit
    }
    return $value
}

puts [counter::increment 3]
