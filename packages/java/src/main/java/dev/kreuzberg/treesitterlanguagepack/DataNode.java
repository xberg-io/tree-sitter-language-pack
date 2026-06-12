// Hand-written — DO NOT EDIT via alef generate (DataNode is not yet in the alef DTO emitter).
// Must match the wire format documented in crates/ts-pack-core/src/intel/types.rs.
package dev.kreuzberg.treesitterlanguagepack;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.fasterxml.jackson.databind.annotation.JsonPOJOBuilder;
import java.util.List;
import org.jspecify.annotations.Nullable;

/**
 * A node in the hierarchical data tree produced by data-format extraction.
 *
 * <p>Populated in {@link ProcessResult#data()} when {@code data_extraction = true} on the
 * {@link ProcessConfig}. The {@code kind} field determines which other fields are meaningful:
 *
 * <ul>
 *   <li>{@link DataNodeKind#KeyValue} — key/value pairs and mapping containers
 *   <li>{@link DataNodeKind#Element} — XML element with tag name in {@code key} and XML attributes
 *   <li>{@link DataNodeKind#Sequence} — positional sequence item with index as {@code key}
 * </ul>
 */
@JsonInclude(JsonInclude.Include.NON_ABSENT)
@JsonDeserialize(builder = DataNode.Builder.class)
@SuppressWarnings("PMD")
public record DataNode(
    @JsonProperty("kind") DataNodeKind kind,
    @Nullable @JsonProperty("key") String key,
    @Nullable @JsonProperty("value") String value,
    @Nullable @JsonProperty("attributes") List<DataAttribute> attributes,
    @Nullable @JsonProperty("children") List<DataNode> children,
    @JsonProperty("span") Span span) {

    public static Builder builder() {
        return new Builder();
    }

    // CPD-OFF
    @com.fasterxml.jackson.annotation.JsonIgnoreProperties(ignoreUnknown = true)
    @JsonPOJOBuilder(withPrefix = "with", buildMethodName = "build")
    public static final class Builder {

        private DataNodeKind kind = DataNodeKind.KeyValue;
        private String key = null;
        private String value = null;
        private List<DataAttribute> attributes = null;
        private List<DataNode> children = null;
        private Span span = null;

        /** Sets the kind field. */
        @JsonProperty("kind")
        public Builder withKind(final DataNodeKind value) {
            this.kind = value;
            return this;
        }

        /** Sets the key field. */
        @JsonProperty("key")
        public Builder withKey(final @Nullable String value) {
            this.key = value;
            return this;
        }

        /** Sets the value field. */
        @JsonProperty("value")
        public Builder withValue(final @Nullable String value) {
            this.value = value;
            return this;
        }

        /** Sets the attributes field. */
        @JsonProperty("attributes")
        public Builder withAttributes(final @Nullable List<DataAttribute> value) {
            this.attributes = value;
            return this;
        }

        /** Sets the children field. */
        @JsonProperty("children")
        public Builder withChildren(final @Nullable List<DataNode> value) {
            this.children = value;
            return this;
        }

        /** Sets the span field. */
        @JsonProperty("span")
        public Builder withSpan(final Span value) {
            this.span = value;
            return this;
        }

        /** Builds the DataNode instance. */
        public DataNode build() {
            return new DataNode(kind, key, value, attributes, children, span);
        }
    }
    // CPD-ON
}
