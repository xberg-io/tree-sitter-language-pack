// Hand-written — DO NOT EDIT via alef generate (DataNodeKind is not yet in the alef DTO emitter).
// Must match the wire format documented in crates/ts-pack-core/src/intel/types.rs.
package dev.kreuzberg.treesitterlanguagepack;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

/**
 * The kind of a data node in the hierarchical extraction tree.
 *
 * Classifies each {@link DataNode} returned when {@code data_extraction} is enabled
 * on {@link ProcessConfig}.
 *
 * <p>Wire format: unit variants serialize as a bare string ({@code "KeyValue"}).
 */
@SuppressWarnings("PMD")
public enum DataNodeKind {
    /** A key/value pair or mapping container (json/toml/properties/yaml/hcl/kdl). */
    KeyValue("KeyValue"),
    /** An XML element with a tag name in {@code key} and attributes populated. */
    Element("Element"),
    /** A positional sequence item (JSON array element, CSV row/cell). */
    Sequence("Sequence");

    /** The wire-format string value. */
    private final String value;

    DataNodeKind(final String value) {
        this.value = value;
    }

    /** Returns the wire-format string value. */
    @JsonValue
    public String getValue() {
        return value;
    }

    /** Creates an instance from a wire-format string. */
    @JsonCreator
    public static DataNodeKind fromValue(final String value) {
        for (DataNodeKind e : values()) {
            if (e.value.equalsIgnoreCase(value)) {
                return e;
            }
        }
        throw new IllegalArgumentException("Unknown DataNodeKind value: " + value);
    }

    /** Returns the wire-format string value (matches JSON serialization). */
    @Override
    public String toString() {
        return value;
    }
}
