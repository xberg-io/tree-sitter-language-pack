// Hand-written — DO NOT EDIT via alef generate (DataNodeKind is not yet in the alef DTO emitter).
// Must match the wire format documented in crates/ts-pack-core/src/intel/types.rs.
@file:Suppress(
    "ktlint:standard:trailing-comma-on-call-site",
    "ktlint:standard:trailing-comma-on-declaration-site",
    "ktlint:standard:max-line-length",
)

package dev.kreuzberg.tslp.android

/**
 * The kind of a data node in the hierarchical extraction tree.
 *
 * Classifies each [DataNode] returned when `data_extraction` is enabled on [ProcessConfig].
 *
 * Wire format: unit variants serialize as a bare string (`"KeyValue"`).
 */
@com.fasterxml.jackson.databind.annotation.JsonDeserialize(using = DataNodeKindDeserializer::class)
@com.fasterxml.jackson.databind.annotation.JsonSerialize(using = DataNodeKindSerializer::class)
sealed class DataNodeKind {
    /** A key/value pair or mapping container (json/toml/properties/yaml/hcl/kdl). */
    object KeyValue : DataNodeKind()

    /** An XML element with a tag name in [DataNode.key] and attributes populated. */
    object Element : DataNodeKind()

    /** A positional sequence item (JSON array element, CSV row/cell). */
    object Sequence : DataNodeKind()
}

private class DataNodeKindDeserializer :
    com.fasterxml.jackson.databind.deser.std.StdDeserializer<DataNodeKind>(
        DataNodeKind::class.java
    ) {
    override fun deserialize(
        parser: com.fasterxml.jackson.core.JsonParser,
        ctx: com.fasterxml.jackson.databind.DeserializationContext,
    ): DataNodeKind {
        val node = parser.codec.readTree<com.fasterxml.jackson.databind.JsonNode>(parser)
        if (node.isTextual) {
            return when (node.asText()) {
                "KeyValue" -> DataNodeKind.KeyValue
                "Element" -> DataNodeKind.Element
                "Sequence" -> DataNodeKind.Sequence
                else ->
                    throw com.fasterxml.jackson.databind.exc.InvalidFormatException(
                        parser,
                        "Unknown DataNodeKind value",
                        node.asText(),
                        DataNodeKind::class.java,
                    )
            }
        }
        throw com.fasterxml.jackson.databind.exc.InvalidFormatException(
            parser,
            "Cannot deserialize DataNodeKind: expected string",
            null,
            DataNodeKind::class.java,
        )
    }
}

private class DataNodeKindSerializer :
    com.fasterxml.jackson.databind.ser.std.StdSerializer<DataNodeKind>(DataNodeKind::class.java) {
    override fun serialize(
        value: DataNodeKind,
        gen: com.fasterxml.jackson.core.JsonGenerator,
        provider: com.fasterxml.jackson.databind.SerializerProvider,
    ) {
        when (value) {
            is DataNodeKind.KeyValue -> gen.writeString("KeyValue")
            is DataNodeKind.Element -> gen.writeString("Element")
            is DataNodeKind.Sequence -> gen.writeString("Sequence")
        }
    }
}
