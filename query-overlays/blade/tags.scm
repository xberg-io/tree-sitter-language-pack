; Sections a Blade template defines with `@section('name')`. The name is the quoted first argument
; the grammar exposes; directives nested in a section body have their own parameters.

(section . (directive) . (parameter) @name) @definition.section

(section . (directive_start) . (parameter) @name) @definition.section
