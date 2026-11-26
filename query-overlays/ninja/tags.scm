; Ninja rules are the recipes build edges invoke, and a build edge's outputs are the targets
; `ninja <target>` runs, so both are functions. Only top-level bindings are file-wide variables;
; one inside a rule, pool or build body sets that block's variable.

; `rule cc`
(rule name: (identifier) @name) @definition.function

; `build greeter: link main.o`, also implicit outputs after `|`
(build (outputs (path) @name)) @definition.function

; The rule a build edge uses: `build greeter: link ...`
(build rule: (identifier) @name) @reference.call

; `default greeter`
(default (targets (path) @name)) @reference.call

; `pool link_pool`
(pool name: (identifier) @name) @definition.module

; `cflags = -O2`
(manifest (let name: (identifier) @name) @definition.variable)

; `$cflags` and `${out}`
(expansion (identifier) @name) @reference.variable

; `include rules.ninja` and `subninja sub/build.ninja`
(include (path) @name) @reference.module

(subninja (path) @name) @reference.module
