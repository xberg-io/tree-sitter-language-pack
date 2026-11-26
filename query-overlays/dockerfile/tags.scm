; A Dockerfile's named blocks are its build stages; `ARG` and `ENV` declare build and runtime
; variables. `COPY --from=<stage>` is not tagged: the grammar keeps the flag as one opaque
; `param` token.

; `FROM golang:1.23 AS builder`
(from_instruction as: (image_alias) @name) @definition.module

; `ARG GO_VERSION=1.23`
(arg_instruction name: (unquoted_string) @name) @definition.variable

; `ENV PORT=8080 LOG_LEVEL=info`, one tag per pair
(env_pair name: (unquoted_string) @name) @definition.variable

; `$PORT` and `${GO_VERSION}`
(expansion (variable) @name) @reference.variable
