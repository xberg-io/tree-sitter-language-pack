; Features and scenarios a Gherkin file defines, named by their titles. Steps reference the step
; definitions that implement them, by their text.

(feature (title) @name) @definition.module

(scenario (title) @name) @definition.function

(step_definition) @name @reference.call
