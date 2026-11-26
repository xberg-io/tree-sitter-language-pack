; MATLAB. Based on the tags.scm of github.com/acristoffers/tree-sitter-matlab, which tagged only
; classes, functions and calls: class methods, the constructor, properties, events and
; enumeration members are added, and base classes are referenced.

(class_definition
  name: (identifier) @name) @definition.class

; `classdef Shape < handle & matlab.mixin.Copyable` names the base classes.
(superclasses
  (property_name (identifier) @name .)) @reference.class

; A method named after its class is the constructor.
(class_definition
  name: (identifier) @_class
  (methods
    (function_definition name: (identifier) @name) @definition.constructor)
  (#eq? @_class @name))

(methods
  (function_definition name: (identifier) @name) @definition.method)

; `properties (Constant)` holds constants; other property blocks hold fields.
(properties
  (attributes (attribute (identifier) @_attribute))
  (property name: (identifier) @name) @definition.constant
  (#eq? @_attribute "Constant"))

(property name: (identifier) @name) @definition.field

(events (identifier) @name) @definition.method

(enumeration (enum . (identifier) @name)) @definition.constant

; Script, file and local functions.
(function_definition
  name: (identifier) @name) @definition.function

(function_call
  name: (identifier) @name) @reference.call

; Command syntax: `disp done`.
(command (command_name) @name) @reference.call
