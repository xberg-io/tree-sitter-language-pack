// Hand-written — DO NOT EDIT via alef generate (DataNode is not yet in the alef DTO emitter).
// Must match the wire format documented in crates/ts-pack-core/src/intel/types.rs.
@file:Suppress(
    "ktlint:standard:trailing-comma-on-call-site",
    "ktlint:standard:trailing-comma-on-declaration-site",
    "ktlint:standard:max-line-length",
)

package dev.kreuzberg.tslp.android

/**
 * A node in the hierarchical data tree produced by data-format extraction.
 *
 * Populated in [ProcessResult.data] when `data_extraction = true` on [ProcessConfig].
 * The [kind] field determines which other fields are meaningful:
 *
 * - [DataNodeKind.KeyValue] — key/value pairs and mapping containers
 * - [DataNodeKind.Element] — XML element with tag name in [key] and XML attributes
 * - [DataNodeKind.Sequence] — positional sequence item with index as [key]
 */
data class DataNode(
    /** Whether this node is a key/value pair, XML element, or sequence item. */
    val kind: DataNodeKind = DataNodeKind.KeyValue,
    /** Key, attribute name, tag name, or positional index. `null` at the document root. */
    val key: String? = null,
    /** Leaf scalar value. `null` for containers (objects, arrays, XML elements with children). */
    val value: String? = null,
    /** Attributes on Element nodes. Empty for KeyValue and Sequence. */
    val attributes: List<DataAttribute> = emptyList(),
    /** Children for nested containers and XML element bodies. */
    val children: List<DataNode> = emptyList(),
    /** Source span covering this node. */
    val span: Span = Span(),
)
