//go:build windows && !tspack_dev

package tspack

// On Windows, CGO_CFLAGS and CGO_LDFLAGS must be set via environment variables
// because ${SRCDIR} expands to paths with colons (e.g. C:\...) that break CGO parsing.
//
// Example:
//   set CGO_CFLAGS=-I/path/to/include
//   set CGO_LDFLAGS=-L/path/to/lib -lts_pack_ffi -lws2_32 -lbcrypt -lntdll -static-libgcc -static-libstdc++

/*
#include "ts_pack.h"
#include <stdlib.h>
#include <stdint.h>
*/
import "C"
