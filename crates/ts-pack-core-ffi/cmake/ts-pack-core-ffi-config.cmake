# ts-pack-core-ffi CMake config-mode find module
#
# Defines the imported target:
#   ts-pack-core-ffi::ts-pack-core-ffi
#
# Usage:
#   find_package(ts-pack-core-ffi REQUIRED)
#   target_link_libraries(myapp PRIVATE ts-pack-core-ffi::ts-pack-core-ffi)

if(TARGET ts-pack-core-ffi::ts-pack-core-ffi)
  return()
endif()

get_filename_component(_FFI_CMAKE_DIR "${CMAKE_CURRENT_LIST_FILE}" PATH)
get_filename_component(_FFI_PREFIX "${_FFI_CMAKE_DIR}/.." ABSOLUTE)

find_library(_FFI_LIBRARY
  NAMES ts_pack_core_ffi libts_pack_core_ffi
  PATHS "${_FFI_PREFIX}/lib"
  NO_DEFAULT_PATH
)
if(NOT _FFI_LIBRARY)
  find_library(_FFI_LIBRARY NAMES ts_pack_core_ffi libts_pack_core_ffi)
endif()

find_path(_FFI_INCLUDE_DIR
  NAMES ts_pack.h
  PATHS "${_FFI_PREFIX}/include"
  NO_DEFAULT_PATH
)
if(NOT _FFI_INCLUDE_DIR)
  find_path(_FFI_INCLUDE_DIR NAMES ts_pack.h)
endif()

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(ts-pack-core-ffi
  REQUIRED_VARS _FFI_LIBRARY _FFI_INCLUDE_DIR
)

if(ts_pack_core_ffi_FOUND)
  set(_FFI_LIB_TYPE UNKNOWN)
  if(_FFI_LIBRARY MATCHES "\\.(dylib|so)$" OR _FFI_LIBRARY MATCHES "\\.so\\.")
    set(_FFI_LIB_TYPE SHARED)
  elseif(_FFI_LIBRARY MATCHES "\\.dll$")
    set(_FFI_LIB_TYPE SHARED)
  elseif(_FFI_LIBRARY MATCHES "\\.(a|lib)$")
    set(_FFI_LIB_TYPE STATIC)
  endif()

  add_library(ts-pack-core-ffi::ts-pack-core-ffi ${_FFI_LIB_TYPE} IMPORTED)
  set_target_properties(ts-pack-core-ffi::ts-pack-core-ffi PROPERTIES
    IMPORTED_LOCATION "${_FFI_LIBRARY}"
    INTERFACE_INCLUDE_DIRECTORIES "${_FFI_INCLUDE_DIR}"
  )

  if(WIN32 AND _FFI_LIB_TYPE STREQUAL "SHARED")
    find_file(_FFI_DLL
      NAMES ts_pack_core_ffi.dll libts_pack_core_ffi.dll
      PATHS "${_FFI_PREFIX}/bin" "${_FFI_PREFIX}/lib"
      NO_DEFAULT_PATH
    )
    if(_FFI_DLL)
      set_target_properties(ts-pack-core-ffi::ts-pack-core-ffi PROPERTIES
        IMPORTED_LOCATION "${_FFI_DLL}"
        IMPORTED_IMPLIB "${_FFI_LIBRARY}"
      )
    endif()
    unset(_FFI_DLL CACHE)
  endif()

  if(APPLE)
    set_property(TARGET ts-pack-core-ffi::ts-pack-core-ffi APPEND PROPERTY
      INTERFACE_LINK_LIBRARIES "-framework CoreFoundation" "-framework Security" pthread)
  elseif(UNIX)
    set_property(TARGET ts-pack-core-ffi::ts-pack-core-ffi APPEND PROPERTY
      INTERFACE_LINK_LIBRARIES pthread dl m)
  elseif(WIN32)
    set_property(TARGET ts-pack-core-ffi::ts-pack-core-ffi APPEND PROPERTY
      INTERFACE_LINK_LIBRARIES ws2_32 userenv bcrypt)
  endif()

  unset(_FFI_LIB_TYPE)
endif()

mark_as_advanced(_FFI_LIBRARY _FFI_INCLUDE_DIR)
unset(_FFI_CMAKE_DIR)
unset(_FFI_PREFIX)
