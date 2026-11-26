" Greet the user from a command.
let g:greeter_name = 'world'
let s:count = 0
const s:prefix = 'Hello'

function! s:Format(name) abort
  return s:prefix . ', ' . a:name . '!'
endfunction

function! greeter#Greet(...) abort
  let l:name = a:0 > 0 ? a:1 : g:greeter_name
  let s:count += 1
  echo s:Format(l:name)
endfunction

command! -nargs=? Greet call greeter#Greet(<f-args>)

augroup greeter
  autocmd!
  autocmd VimEnter * call greeter#Greet()
augroup END
