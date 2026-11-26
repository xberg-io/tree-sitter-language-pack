use std/log

const PREFIX = "/usr/local"
let targets = [api worker]
mut attempts = 0

module tools {
    export def greet [name: string] {
        $"hello ($name)"
    }
}

alias ll = ls -l

def "build target" [name: string] {
    let out = $"($PREFIX)/bin/($name)"
    log info $"building ($name) into ($out)"
}

export def deploy [] {
    for t in $targets {
        build target $t
    }
}

extern "git push" [remote?: string]

extern rg [pattern: string, path?: string]

module "release tools" {
    export def bump [] { "bumped" }
}

alias "gs" = git status

use "tools.nu" greet
