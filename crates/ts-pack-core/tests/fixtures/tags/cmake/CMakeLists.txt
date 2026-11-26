cmake_minimum_required(VERSION 3.20)
project(greeter LANGUAGES C)

set(GREETER_SOURCES src/main.c src/greet.c)
option(GREETER_WERROR "Treat warnings as errors" OFF)

function(greeter_add_warnings target)
  set(flags -Wall -Wextra)
  target_compile_options(${target} PRIVATE ${flags})
endfunction()

macro(greeter_install target)
  install(TARGETS ${target} DESTINATION bin)
endmacro()

add_library(greet STATIC src/greet.c)
add_executable(greeter ${GREETER_SOURCES})
greeter_add_warnings(greeter)
greeter_install(greeter)
