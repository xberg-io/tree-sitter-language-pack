package app.authz

import rego.v1

default allow := false

admins := {"alice", "bob"}

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
