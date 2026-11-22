// Probe library for tests/typst_vec_u32_pop.rs. Not a test target of its own:
// cargo only builds `tests/*.rs`, and the Rust test compiles this file into a
// shared library at run time.
//
// `vec_u32_pop` is `static` and every caller in the scanner discards its return
// value, so no built grammar library exports it and no parse can observe what
// it returns. Compiling the patched scanner.c into this translation unit is the
// only way to see the value itself rather than waiting for the one-past-the-end
// read to land on an unmapped page. ~keep
#include <stddef.h>
#include <stdint.h>

#include SCANNER_UNDER_TEST

#define TSLP_PROBE_OK 0
#define TSLP_PROBE_SETUP_FAILED 1

// Pushes the three `seed` values, then pops three times, recording each
// returned value and the length left behind.
//
// Before every pop the first slot past the logical end is overwritten with
// `sentinel`. `vec_u32_push` over-allocates (cap == len + 8), so that slot is
// inside the allocation and writing it is defined. A pop that indexes
// `vec[len]` instead of `vec[len - 1]` therefore returns `sentinel` on every
// platform, rather than depending on what happens to follow the heap block.
// ~keep
int tslp_vec_u32_pop_probe(uint32_t sentinel, const uint32_t *seed, uint32_t *values,
                           size_t *lengths) {
    struct vec_u32 vector = vec_u32_new();
    for (size_t index = 0; index < 3; index += 1) {
        vec_u32_push(&vector, seed[index]);
    }
    if (vector.len != 3 || vector.cap <= vector.len) {
        vec_u32_drop(vector);
        return TSLP_PROBE_SETUP_FAILED;
    }
    for (size_t index = 0; index < 3; index += 1) {
        vector.vec[vector.len] = sentinel;
        values[index] = vec_u32_pop(&vector);
        lengths[index] = vector.len;
    }
    vec_u32_drop(vector);
    return TSLP_PROBE_OK;
}

// Pops the single pushed element, then pops again from the now-empty vector.
//
// The second pop is on a vector whose buffer is still allocated, so both the
// guarded and the unguarded form stay inside the allocation and neither faults.
// What differs is the length: a pre-decrement without a guard wraps it to
// SIZE_MAX. ~keep
int tslp_vec_u32_pop_empty_probe(uint32_t seed, uint32_t *value, size_t *length) {
    struct vec_u32 vector = vec_u32_new();
    vec_u32_push(&vector, seed);
    (void)vec_u32_pop(&vector);
    if (vector.len != 0 || vector.vec == NULL) {
        vec_u32_drop(vector);
        return TSLP_PROBE_SETUP_FAILED;
    }
    *value = vec_u32_pop(&vector);
    *length = vector.len;
    vec_u32_drop(vector);
    return TSLP_PROBE_OK;
}
