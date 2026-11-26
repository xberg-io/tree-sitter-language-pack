cmake_minimum_required(VERSION 3.20)
project(greeter LANGUAGES C)

set(GREETER_SOURCES src/main.c src/greet.c)
set(GREETER_VERSION "1.2.0" CACHE STRING "Greeter version")
option(GREETER_WERROR "Treat warnings as errors" OFF)
set(${PROJECT_NAME}_EXTRA_FLAGS -O2)

function(greeter_add_warnings target)
  set(flags -Wall -Wextra)
  target_compile_options(${target} PRIVATE ${flags})
endfunction()

macro(greeter_install target)
  install(TARGETS ${target} DESTINATION bin)
endmacro()

add_library(greet STATIC src/greet.c)
add_library(${PROJECT_NAME}_core OBJECT src/core.c)
add_executable(greeter ${GREETER_SOURCES})
add_custom_target(docs COMMAND doxygen)

if(GREETER_WERROR)
  greeter_add_warnings(greeter)
endif()

foreach(lib IN ITEMS greet)
  target_link_libraries(greeter PRIVATE ${lib})
endforeach()

greeter_install(greeter)
