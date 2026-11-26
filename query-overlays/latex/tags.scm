; Sectioning commands, labels, and the commands and environments a LaTeX document defines, plus
; the references and citations that point at labels and bibliography keys.

[
  (part text: (curly_group (text) @name))
  (chapter text: (curly_group (text) @name))
  (section text: (curly_group (text) @name))
  (subsection text: (curly_group (text) @name))
  (subsubsection text: (curly_group (text) @name))
  (paragraph text: (curly_group (text) @name))
  (subparagraph text: (curly_group (text) @name))
] @definition.section

; `\label{sec:intro}`
(label_definition name: (curly_group_label (label) @name)) @definition.label

; `\newcommand{\pack}{...}`
(new_command_definition
  declaration: (curly_group_command_name (command_name) @name)) @definition.macro

; `\newenvironment{note}{...}{...}`
(environment_definition name: (curly_group_text (text) @name)) @definition.macro

; `\ref{sec:usage}`, `\cref{a,b}`
(label_reference names: (curly_group_label_list (label) @name)) @reference.label

; `\cite{knuth84}`
(citation keys: (curly_group_text_list (text) @name)) @reference.label
