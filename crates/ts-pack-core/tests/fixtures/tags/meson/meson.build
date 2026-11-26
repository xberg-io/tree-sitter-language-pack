project('greeter', 'c', version : '1.2.0')

cc = meson.get_compiler('c')
greet_sources = files('src/greet.c')
greet_sources += files('src/format.c')

greet_lib = static_library('greet', greet_sources,
  include_directories : include_directories('include'))

greeter = executable('greeter', 'src/main.c',
  link_with : greet_lib,
  install : true)

test('greeter-runs', greeter)
