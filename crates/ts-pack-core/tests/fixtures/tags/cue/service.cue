package deploy

import "strings"

#Port: int & >0 & <65536

#Service: {
	name:  string
	port:  #Port
	image: string
	#Probe: {path: string | *"/healthz"}
	probe: #Probe
}

let _registry = "ghcr.io/example"

api: #Service & {
	name:  "api"
	port:  8080
	image: "\(_registry)/api:\(strings.ToLower("V1"))"
}
