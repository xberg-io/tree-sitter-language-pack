# Prompt and helper setup for interactive shells.
PROMPT_COLOR=cyan
typeset -g HISTSIZE=10000
export EDITOR=nvim
A=1 B=2

alias ll='ls -lh'

git_branch() {
  local ref
  ref=$(git symbolic-ref --short HEAD 2>/dev/null) || return
  print -n "($ref)"
}

function precmd {
  PROMPT="%F{$PROMPT_COLOR}%~%f $(git_branch) %# "
}
