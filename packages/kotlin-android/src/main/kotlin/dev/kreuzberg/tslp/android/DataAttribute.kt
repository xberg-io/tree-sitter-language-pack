// Hand-written — DO NOT EDIT via alef generate (DataAttribute is not yet in the alef DTO emitter).
// Must match the wire format documented in crates/ts-pack-core/src/intel/types.rs.
@file:Suppress(
    "ktlint:standard:trailing-comma-on-call-site",
    "ktlint:standard:trailing-comma-on-declaration-site",
    "ktlint:standard:max-line-length",
)

package dev.kreuzberg.tslp.android

/**
 * An XML-style attribute attached to a [DataNode] of kind [DataNodeKind.Element].
 *
 * Always empty for [DataNodeKind.KeyValue] and [DataNodeKind.Sequence] nodes.
 */
data class DataAttribute(
    /** Attribute name (e.g. `"class"`, `"href"`). */
    val name: String = "",
    /** Attribute value as a raw string (quotes stripped). */
    val value: String = "",
    /** Source span covering the entire `name="value"` attribute token. */
    val span: Span = Span(),
)
