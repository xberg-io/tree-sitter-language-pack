package app.authz

import rego.v1

admins := {"alice", "bob"}

default allow := false

allow if {
	is_admin
}

allow if {
	input.method == "GET"
	count(violations) == 0
}

is_admin if input.user in admins

violations contains msg if {
	not input.user
	msg := "missing user"
}

double(x) := y if {
	y := x * 2
}
