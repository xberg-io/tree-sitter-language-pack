# Sum the third column per key and print a sorted report.
function trim(s) {
    gsub(/^[ \t]+|[ \t]+$/, "", s)
    return s
}

function fmt_total(key, total) {
    return sprintf("%-20s %10.2f", trim(key), total)
}

func is_header(line) {
    return line ~ /^key,/
}

BEGIN { FS = "," }

is_header($0) { next }

NR > 1 { totals[$1] += $3 }

END {
    for (k in totals)
        print fmt_total(k, totals[k])
}
