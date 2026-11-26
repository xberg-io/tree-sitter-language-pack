; Makefile targets are its recipes, so they are functions. Special targets (`.PHONY`,
; `.SUFFIXES`) and pattern rules (`%.o: %.c`) name no recipe anyone invokes and are skipped.

; `greeter: $(OBJS)` and `all clean:`
(rule
  (targets (word) @name)
  (#not-match? @name "^\\.|%")) @definition.function

; `CC ?= cc`, `CFLAGS := -O2`, `override X = 1`, `export X = 1`, also inside conditionals and
; target-specific ones (`debug: CFLAGS += -g`), which the grammar shapes the same way
(variable_assignment name: (word) @name) @definition.variable

; `UNAME != uname -s`
(shell_assignment name: (word) @name) @definition.variable

; `define link ... endef` is a multi-line variable expanded with `$(call link)`.
(define_directive name: (word) @name) @definition.macro

; `$(CC)` and `${CFLAGS}`
(variable_reference (word) @name) @reference.variable

; `include config.mk` and `-include deps.mk`
(include_directive filenames: (list (word) @name)) @reference.module

; `$(SRCS:.c=.o)`
(substitution_reference text: (word) @name) @reference.variable

; `$(call link,args)`
(function_call
  function: "call"
  (arguments . argument: (text) @name)) @reference.call
