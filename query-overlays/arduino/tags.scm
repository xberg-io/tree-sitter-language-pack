; Arduino sketches are C++: tree-sitter-arduino (github.com/ObserverOfTime/tree-sitter-arduino)
; extends tree-sitter-cpp, so this is query-overlays/cpp/tags.scm, which grew from the vendored
; tree-sitter-cpp queries/tags.scm. The vendored arduino query tagged only functions and calls.
; ~keep A member declared in its class and defined out of line (`double Point::norm() const {`)
; ~keep names one method: only definitions tag, plus pure virtual declarations, which have no
; ~keep definition. A function declared but defined in another file has no tag in this one.

; Classes, structs and unions with a body; `class Foo;` and `struct Foo p;` are uses.
(class_specifier
  name: (type_identifier) @name
  body: (_)) @definition.class

(struct_specifier
  name: (type_identifier) @name
  body: (_)) @definition.class

(union_specifier
  name: (type_identifier) @name
  body: (_)) @definition.class

(enum_specifier
  name: (type_identifier) @name
  body: (_)) @definition.enum

(enumerator
  name: (identifier) @name) @definition.constant

; `namespace geo {` and `namespace detail::math {`; the nested form tags its qualified name.
(namespace_definition
  name: [
    (namespace_identifier)
    (nested_namespace_specifier)
  ] @name) @definition.module

(type_definition
  declarator: (type_identifier) @name) @definition.type

; `using Name = std::string;`
(alias_declaration
  name: (type_identifier) @name) @definition.type

(preproc_def
  name: (identifier) @name) @definition.macro

(preproc_function_def
  name: (identifier) @name) @definition.macro

; Overloads name one function: the first definition in a class body or namespace tags, later
; ones are claimed. Out-of-line overloads (`Point::scale` twice) each tag.
([
  (field_declaration_list
    (function_definition
      declarator: (function_declarator
        declarator: [
          (field_identifier)
          (identifier)
        ] @_first))
    (function_definition
      declarator: (function_declarator
        declarator: [
          (field_identifier)
          (identifier)
        ] @ignore)))
  (translation_unit
    (function_definition
      declarator: (function_declarator
        declarator: (identifier) @_first))
    (function_definition
      declarator: (function_declarator
        declarator: (identifier) @ignore)))
  (declaration_list
    (function_definition
      declarator: (function_declarator
        declarator: (identifier) @_first))
    (function_definition
      declarator: (function_declarator
        declarator: (identifier) @ignore)))
]
  (#eq? @_first @ignore))

; Constructors: `Point(double x) { ... }` in the class body and `Circle::Circle(double r) { ... }`
; out of line.
(field_declaration_list
  (function_definition
    declarator: (function_declarator
      declarator: (identifier) @name)) @definition.constructor)

(function_definition
  declarator: (function_declarator
    declarator: (qualified_identifier
      scope: (namespace_identifier) @_scope
      name: (identifier) @name))
  (#eq? @_scope @name)) @definition.constructor

; Methods: defined in the class body, out of line through their class, or pure virtual.
(function_definition
  declarator: (function_declarator
    declarator: (field_identifier) @name)) @definition.method

(function_definition
  declarator: (reference_declarator
    (function_declarator
      declarator: (field_identifier) @name))) @definition.method

(function_definition
  declarator: (function_declarator
    declarator: (qualified_identifier
      scope: (namespace_identifier) @_scope
      name: (identifier) @name))
  (#not-eq? @_scope @name)) @definition.method

; `virtual double area() const = 0;`
(field_declaration
  declarator: (function_declarator
    declarator: (field_identifier) @name)
  default_value: (number_literal) @_zero
  (#eq? @_zero "0")) @definition.method

; Free functions, including function templates and pointer or reference returns.
(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name)) @definition.function

(function_definition
  declarator: [
    (pointer_declarator
      declarator: (function_declarator
        declarator: (identifier) @name))
    (reference_declarator
      (function_declarator
        declarator: (identifier) @name))
  ]) @definition.function

; Data members.
(field_declaration
  declarator: [
    (field_identifier) @name
    (array_declarator
      declarator: (field_identifier) @name)
  ]) @definition.field

; File- and namespace-scope variables; `const` and `constexpr` ones are constants. `extern`
; declarations name a variable defined elsewhere, and declarations in function bodies are locals.
([
  (translation_unit
    (declaration
      (storage_class_specifier) @_storage
      declarator: [
        (identifier) @ignore
        (init_declarator
          declarator: (identifier) @ignore)
      ]))
  (declaration_list
    (declaration
      (storage_class_specifier) @_storage
      declarator: [
        (identifier) @ignore
        (init_declarator
          declarator: (identifier) @ignore)
      ]))
]
  (#eq? @_storage "extern"))

([
  (translation_unit
    (declaration
      (type_qualifier) @_qualifier
      declarator: (init_declarator
        declarator: (identifier) @name)) @definition.constant)
  (declaration_list
    (declaration
      (type_qualifier) @_qualifier
      declarator: (init_declarator
        declarator: (identifier) @name)) @definition.constant)
]
  (#any-of? @_qualifier "const" "constexpr"))

[
  (translation_unit
    (declaration
      declarator: [
        (identifier) @name
        (pointer_declarator
          declarator: (identifier) @name)
        (init_declarator
          declarator: [
            (identifier) @name
            (pointer_declarator
              declarator: (identifier) @name)
          ])
      ]) @definition.variable)
  (declaration_list
    (declaration
      declarator: [
        (identifier) @name
        (pointer_declarator
          declarator: (identifier) @name)
        (init_declarator
          declarator: [
            (identifier) @name
            (pointer_declarator
              declarator: (identifier) @name)
          ])
      ]) @definition.variable)
]

; Base classes: `class Circle : public Shape` and `class Registry : public std::vector<T>`.
(base_class_clause
  [
    (type_identifier) @name
    (qualified_identifier
      name: (type_identifier) @name)
    (template_type
      name: (type_identifier) @name)
    (qualified_identifier
      name: (template_type
        name: (type_identifier) @name))
  ]) @reference.implementation

; Calls: `clamp(...)`, `geo::clamp(...)`, `detail::math::hypot2(...)`, `shape->area()`,
; `circle.area()` and `geo::clamp<int>(...)`; the cast operators are keywords, not calls.
((call_expression
  function: [
    (identifier) @name
    (qualified_identifier
      name: [
        (identifier) @name
        (qualified_identifier
          name: (identifier) @name)
        (template_function
          name: (identifier) @name)
      ])
    (field_expression
      field: (field_identifier) @name)
    (template_function
      name: (identifier) @name)
  ]) @reference.call
  (#not-any-of? @name "static_cast" "dynamic_cast" "const_cast" "reinterpret_cast"))
