/*
 * ts-pack deterministic wide-ctype shim.
 *
 * ~keep Vendored grammar scanners call the libc wide-ctype classifiers
 * (iswalpha/iswalnum/iswspace/...) on Unicode code points. Those functions are
 * locale-sensitive and, for non-ASCII code points, disagree between BSD libc
 * (macOS) and glibc (Linux) even under the same locale name — e.g.
 * iswspace(U+00A0) and iswalpha(U+0670) differ across platforms. That makes the
 * per-platform parser .so/.dylib artifacts tokenize the same source into
 * different parse trees, which is a cross-platform non-determinism bug for any
 * consumer that reindents/formats from the tree.
 *
 * This header is force-included (clang/gcc `-include`, MSVC `/FI`) ahead of
 * every scanner translation unit. It redirects the libc entry points to small
 * wrappers backed by utf8proc's frozen, in-source Unicode tables, so the result
 * is identical on every platform *by construction* (same source, no libc, no
 * locale) while preserving non-ASCII Unicode correctness.
 *
 * Conflict-safety: the real libc declarations are pulled in FIRST (below), so
 * their prototypes are emitted under the real names before any redirect macro
 * exists; a scanner's own later `#include <wctype.h>`/`<cwctype>` then hits the
 * include guard and is a no-op — no prototype/macro clash. The redirects are
 * *function-like* macros, so a C++ `using std::iswspace;` (identifier not
 * followed by `(`) is left untouched and only call sites expand.
 */
#ifndef TS_PACK_CTYPE_SHIM_H
#define TS_PACK_CTYPE_SHIM_H

/* ~keep Emit the real libc prototypes under their real names before
 * redirecting. Only <wctype.h>/<cwctype> — deliberately NOT <wchar.h>, which
 * transitively leaks <stdio.h>'s EOF macro and breaks scanners that use EOF as
 * an identifier (e.g. caddy's `enum { EOF }`). <wctype.h> alone supplies wint_t
 * + prototypes. */
#ifdef __cplusplus
#include <cwctype>
#else
#include <wctype.h>
#endif

/* ~keep wasm32/wasi only: wasi-libc's <wchar.h> unconditionally #undefs and
 * re-#defines iswdigit/iswalpha/... (as "0 ? realcall : fastpath" trampoline
 * macros) inside its own top-level include guard. Scanners that later do their
 * own `#include <wchar.h>` (most tree-sitter scanner templates do) re-run that
 * body and silently clobber our redirects below with no redefinition warning
 * (it's preceded by an #undef), leaving call sites referencing the real,
 * wasm32-undefined `iswdigit` — surfaces as a `require("env")` import in the
 * wasm-bindgen glue. Including <wchar.h> here first burns its include guard so
 * later scanner includes are no-ops. Gated on __wasi__ (defined only for the
 * wasm32 sysroot build, see build.rs) because native libcs' <wchar.h> can
 * transitively leak <stdio.h>'s EOF macro (breaks scanners using EOF as an
 * identifier, e.g. caddy's `enum { EOF }` — see above); wasi-libc's does not.
 */
#ifdef __wasi__
#ifdef __cplusplus
#include <cwchar>
#else
#include <wchar.h>
#endif
#endif

/* ~keep wasm32 has no process to exit: wasi-libc's exit() is a
 * `wasi_snapshot_preview1` proc_exit call and wasm32-unknown-unknown has no
 * libc at all, so a scanner's `exit(1)` (new in tree-sitter-swift 171fa3b, OOM
 * path of scanner_create) is left unresolved and rust-lld turns it into an
 * `env.exit` import. The module then fails to load with `Cannot find module
 * 'env'`. <stdlib.h> is included first so the real prototype is emitted before
 * the macro exists. C only: C++ scanners would qualify it as std::exit. */
#if defined(__wasi__) && !defined(__cplusplus)
#include <stdlib.h>
#define exit(code) __builtin_trap()
#endif

#include "utf8proc.h"

/* ~keep Unicode range guard: utf8proc classifies out-of-range as unassigned. */
#define TS_PACK_CP_MAX 0x10FFFF

static inline int ts_pack__cat(utf8proc_int32_t c) {
  if (c < 0 || c > TS_PACK_CP_MAX) {
    return UTF8PROC_CATEGORY_CN;
  }
  return (int)utf8proc_category(c);
}

static inline int ts_pack_iswalpha(utf8proc_int32_t c) {
  switch (ts_pack__cat(c)) {
  case UTF8PROC_CATEGORY_LU:
  case UTF8PROC_CATEGORY_LL:
  case UTF8PROC_CATEGORY_LT:
  case UTF8PROC_CATEGORY_LM:
  case UTF8PROC_CATEGORY_LO:
    return 1;
  default:
    return 0;
  }
}

/* ~keep iswdigit/iswxdigit are ASCII-only by the C standard (locale-invariant),
 * so a fixed ASCII test both matches every libc and stays deterministic. */
static inline int ts_pack_iswdigit(utf8proc_int32_t c) {
  return c >= '0' && c <= '9';
}

static inline int ts_pack_iswxdigit(utf8proc_int32_t c) {
  return (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') ||
         (c >= 'A' && c <= 'F');
}

/* ~keep alnum keeps Unicode identifier continuation working: letters plus all
 * numeric categories (decimal, letter-number, other-number). */
static inline int ts_pack_iswalnum(utf8proc_int32_t c) {
  switch (ts_pack__cat(c)) {
  case UTF8PROC_CATEGORY_LU:
  case UTF8PROC_CATEGORY_LL:
  case UTF8PROC_CATEGORY_LT:
  case UTF8PROC_CATEGORY_LM:
  case UTF8PROC_CATEGORY_LO:
  case UTF8PROC_CATEGORY_ND:
  case UTF8PROC_CATEGORY_NL:
  case UTF8PROC_CATEGORY_NO:
    return 1;
  default:
    return 0;
  }
}

/* ~keep Frozen Unicode White_Space property set (identical on every platform).
 */
static inline int ts_pack_iswspace(utf8proc_int32_t c) {
  switch (c) {
  case 0x09:
  case 0x0A:
  case 0x0B:
  case 0x0C:
  case 0x0D:
  case 0x20:
  case 0x85:
  case 0xA0:
  case 0x1680:
  case 0x2000:
  case 0x2001:
  case 0x2002:
  case 0x2003:
  case 0x2004:
  case 0x2005:
  case 0x2006:
  case 0x2007:
  case 0x2008:
  case 0x2009:
  case 0x200A:
  case 0x2028:
  case 0x2029:
  case 0x202F:
  case 0x205F:
  case 0x3000:
    return 1;
  default:
    return 0;
  }
}

static inline int ts_pack_iswupper(utf8proc_int32_t c) {
  return ts_pack__cat(c) == UTF8PROC_CATEGORY_LU;
}

static inline int ts_pack_iswlower(utf8proc_int32_t c) {
  return ts_pack__cat(c) == UTF8PROC_CATEGORY_LL;
}

static inline int ts_pack_iswpunct(utf8proc_int32_t c) {
  switch (ts_pack__cat(c)) {
  case UTF8PROC_CATEGORY_PC:
  case UTF8PROC_CATEGORY_PD:
  case UTF8PROC_CATEGORY_PS:
  case UTF8PROC_CATEGORY_PE:
  case UTF8PROC_CATEGORY_PI:
  case UTF8PROC_CATEGORY_PF:
  case UTF8PROC_CATEGORY_PO:
  case UTF8PROC_CATEGORY_SM:
  case UTF8PROC_CATEGORY_SC:
  case UTF8PROC_CATEGORY_SK:
  case UTF8PROC_CATEGORY_SO:
    return 1;
  default:
    return 0;
  }
}

static inline int ts_pack_iswcntrl(utf8proc_int32_t c) {
  return ts_pack__cat(c) == UTF8PROC_CATEGORY_CC;
}

static inline int ts_pack_iswblank(utf8proc_int32_t c) {
  return c == '\t' || ts_pack__cat(c) == UTF8PROC_CATEGORY_ZS;
}

static inline int ts_pack_iswprint(utf8proc_int32_t c) {
  switch (ts_pack__cat(c)) {
  case UTF8PROC_CATEGORY_CC:
  case UTF8PROC_CATEGORY_CF:
  case UTF8PROC_CATEGORY_CS:
  case UTF8PROC_CATEGORY_CN:
    return 0;
  default:
    return 1;
  }
}

static inline int ts_pack_iswgraph(utf8proc_int32_t c) {
  return ts_pack_iswprint(c) && !ts_pack_iswspace(c);
}

static inline utf8proc_int32_t ts_pack_towupper(utf8proc_int32_t c) {
  if (c < 0 || c > TS_PACK_CP_MAX) {
    return c;
  }
  return utf8proc_toupper(c);
}

static inline utf8proc_int32_t ts_pack_towlower(utf8proc_int32_t c) {
  if (c < 0 || c > TS_PACK_CP_MAX) {
    return c;
  }
  return utf8proc_tolower(c);
}

/*
 * ~keep Function-like redirects. Defined AFTER the real prototypes so there is
 * no clash. iswblank is redirected for C only: the sole C++ user (norg) calls
 * it as `std::iswblank(...)`, and a macro would rewrite that to a non-existent
 * `std::ts_pack_iswblank` — leaving it on libc there is harmless (blank is
 * near-ASCII) and keeps that translation unit compiling.
 */
#define iswalpha(c) ts_pack_iswalpha((c))
#define iswalnum(c) ts_pack_iswalnum((c))
#define iswdigit(c) ts_pack_iswdigit((c))
#define iswxdigit(c) ts_pack_iswxdigit((c))
#define iswspace(c) ts_pack_iswspace((c))
#define iswupper(c) ts_pack_iswupper((c))
#define iswlower(c) ts_pack_iswlower((c))
#define iswpunct(c) ts_pack_iswpunct((c))
#define iswcntrl(c) ts_pack_iswcntrl((c))
#define iswprint(c) ts_pack_iswprint((c))
#define iswgraph(c) ts_pack_iswgraph((c))
#define towupper(c) ts_pack_towupper((c))
#define towlower(c) ts_pack_towlower((c))
#ifndef __cplusplus
#define iswblank(c) ts_pack_iswblank((c))
#endif

/*
 * ~keep std::-qualified-call hazard: preprocessing is token-based, so a C++
 * scanner calling e.g. `std::iswspace(c)` macro-expands to
 * `std::ts_pack_iswspace((c))` — a name that doesn't exist in namespace std
 * unless we put it there. tree-sitter-norg's scanner.cc only qualifies
 * iswblank this way today (see the precedent above), but nothing stops a
 * future grammar from qualifying any of the others, so every redirected name
 * gets a `using` declaration here up front. iswblank is deliberately
 * excluded: it is not redirected in C++ (see above), so `std::iswblank`
 * still resolves to the real libc function and needs no help here.
 */
#ifdef __cplusplus
namespace std {
using ::ts_pack_iswalnum;
using ::ts_pack_iswalpha;
using ::ts_pack_iswcntrl;
using ::ts_pack_iswdigit;
using ::ts_pack_iswgraph;
using ::ts_pack_iswlower;
using ::ts_pack_iswprint;
using ::ts_pack_iswpunct;
using ::ts_pack_iswspace;
using ::ts_pack_iswupper;
using ::ts_pack_iswxdigit;
using ::ts_pack_towlower;
using ::ts_pack_towupper;
} // namespace std
#endif

#endif /* TS_PACK_CTYPE_SHIM_H */
