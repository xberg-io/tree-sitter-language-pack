; Elixir tags: the vendored tags.scm of elixir-lang/tree-sitter-elixir, with typed definition
; kinds (protocols, structs, macros, callbacks, types, module attributes), multi-clause functions
; tagged once, and implementation references.

; Definitions

; * multi-clause functions and macros: a clause directly following a clause of the same name is
;   claimed without a tag, so only the first clause is tagged. Listed first so it wins.
((call
  target: (identifier) @_kind
  (arguments
    [
      (identifier) @_first
      (call target: (identifier) @_first)
      (binary_operator
        left: (call target: (identifier) @_first)
        operator: "when")
    ]))
  .
  (call
    target: (identifier) @_next_kind
    (arguments
      [
        (identifier) @ignore
        (call target: (identifier) @ignore)
        (binary_operator
          left: (call target: (identifier) @ignore)
          operator: "when")
      ]))
  (#any-of? @_kind "def" "defp" "defmacro" "defmacrop" "defguard" "defguardp" "defn" "defnp")
  (#eq? @_kind @_next_kind)
  (#eq? @_first @ignore))

; * a module that defines a struct or an exception is that struct: tagged as a class at its
;   module name (the struct has no name of its own, and its fields are atoms that include the
;   leading colon).
(call
  target: (identifier) @_defmodule
  (arguments (alias) @name)
  (do_block
    (call
      target: (identifier) @_defstruct))
  (#eq? @_defmodule "defmodule")
  (#any-of? @_defstruct "defstruct" "defexception")) @definition.class

; * modules
(call
  target: (identifier) @_defmodule
  (arguments (alias) @name)
  (#eq? @_defmodule "defmodule")) @definition.module

; * protocols and their functions
(call
  target: (identifier) @_defprotocol
  (arguments (alias) @name)
  (#eq? @_defprotocol "defprotocol")) @definition.interface

(call
  target: (identifier) @_defprotocol
  (do_block
    (call
      target: (identifier) @_def
      (arguments
        [
          (identifier) @name
          (call target: (identifier) @name)
        ])) @definition.method)
  (#eq? @_defprotocol "defprotocol")
  (#eq? @_def "def"))

; * protocol implementations: `defimpl Size, for: Circle`
(call
  target: (identifier) @_defimpl
  (arguments
    .
    (alias) @name)
  (#eq? @_defimpl "defimpl")) @reference.implementation

; * macros and guards (guards are macros)
(call
  target: (identifier) @_kind
  (arguments
    [
      ; zero-arity macros with no parentheses
      (identifier) @name
      ; regular macro clause
      (call target: (identifier) @name)
      ; macro clause with a guard clause
      (binary_operator
        left: (call target: (identifier) @name)
        operator: "when")
    ])
  (#any-of? @_kind "defmacro" "defmacrop" "defguard" "defguardp")) @definition.macro

; * functions
(call
  target: (identifier) @_kind
  (arguments
    [
      ; zero-arity functions with no parentheses
      (identifier) @name
      ; regular function clause
      (call target: (identifier) @name)
      ; function clause with a guard clause
      (binary_operator
        left: (call target: (identifier) @name)
        operator: "when")
    ])
  (#any-of? @_kind "def" "defp" "defdelegate" "defn" "defnp")) @definition.function

; * behaviour callbacks: `@callback init(term) :: result` declares a member of the behaviour's
;   interface; `@macrocallback` declares a macro.
(unary_operator
  operator: "@"
  operand: (call
    target: (identifier) @_attribute
    (arguments
      (binary_operator
        left: [
          (identifier) @name
          (call target: (identifier) @name)
        ]
        operator: "::")))
  (#eq? @_attribute "callback")) @definition.method

(unary_operator
  operator: "@"
  operand: (call
    target: (identifier) @_attribute
    (arguments
      (binary_operator
        left: [
          (identifier) @name
          (call target: (identifier) @name)
        ]
        operator: "::")))
  (#eq? @_attribute "macrocallback")) @definition.macro

; * types: `@type t :: ...`, `@typep pair(a) :: ...`, `@opaque id :: ...`
(unary_operator
  operator: "@"
  operand: (call
    target: (identifier) @_attribute
    (arguments
      (binary_operator
        left: [
          (identifier) @name
          (call target: (identifier) @name)
        ]
        operator: "::")))
  (#any-of? @_attribute "type" "typep" "opaque")) @definition.type

; * function specs name a function defined elsewhere: claim the name without a tag.
(unary_operator
  operator: "@"
  operand: (call
    target: (identifier) @_attribute
    (arguments
      (binary_operator
        left: [
          (call target: (identifier) @ignore)
          (binary_operator
            left: (call target: (identifier) @ignore)
            operator: "when")
        ])))
  (#any-of? @_attribute "spec"))

; * module attributes with a value are compile-time constants: `@max_size 10`. The grammar's
;   operand node spells the name without the `@`. Built-in attributes are not constants.
(unary_operator
  operator: "@"
  operand: (call
    target: (identifier) @name
    (arguments))
  (#not-match? @name "^(moduledoc|doc|typedoc|spec|type|typep|opaque|callback|macrocallback|optional_callbacks|impl|behaviour|derive|enforce_keys|dialyzer|compile|before_compile|after_compile|after_verify|on_definition|on_load|external_resource|deprecated|since|vsn|file)$")) @definition.constant

; References

; * adopted behaviours: `@behaviour GenServer`
(unary_operator
  operator: "@"
  operand: (call
    target: (identifier) @_attribute
    (arguments (alias) @name))
  (#eq? @_attribute "behaviour")) @reference.implementation

; ignore calls to kernel/special-forms keywords
(call
  target: (identifier) @ignore
  (#any-of? @ignore "def" "defp" "defdelegate" "defguard" "defguardp" "defmacro" "defmacrop" "defn" "defnp" "defmodule" "defprotocol" "defimpl" "defstruct" "defexception" "defoverridable" "alias" "case" "cond" "else" "for" "if" "import" "quote" "raise" "receive" "require" "reraise" "super" "throw" "try" "unless" "unquote" "unquote_splicing" "use" "with"))

; ignore module attributes
(unary_operator
  operator: "@"
  operand: (call
    target: (identifier) @ignore))

; * function call: local, or remote with an argument list (`person.name` without one reads a
;   map or struct field)
(call
  target: (identifier) @name) @reference.call

(call
  target: (dot
    right: (identifier) @name)
  (arguments)) @reference.call

; * pipe into function call
(binary_operator
  operator: "|>"
  right: (identifier) @name) @reference.call

; * modules
(alias) @name @reference.module
