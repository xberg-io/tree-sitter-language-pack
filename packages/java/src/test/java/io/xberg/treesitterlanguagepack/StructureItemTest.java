package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;

import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.List;
import org.junit.jupiter.api.Test;

class StructureItemTest {

    private static final Span SAMPLE_SPAN = new Span(0, 20, 0, 0, 2, 0);

    @Test
    void shouldExposeAllAccessors() {
        Span bodySpan = new Span(5, 20, 1, 0, 2, 0);
        StructureItem item = new StructureItem(
            StructureKind.FUNCTION, "main", "public", SAMPLE_SPAN, List.of(),
            List.of("Override"), "runs the program", "fn main()", bodySpan
        );

        assertEquals(StructureKind.FUNCTION, item.kind());
        assertEquals("main", item.name());
        assertEquals("public", item.visibility());
        assertEquals(SAMPLE_SPAN, item.span());
        assertEquals(List.of(), item.children());
        assertEquals(List.of("Override"), item.decorators());
        assertEquals("runs the program", item.docComment());
        assertEquals("fn main()", item.signature());
        assertEquals(bodySpan, item.bodySpan());
    }

    @Test
    void shouldNormalizeNullCollectionsToEmptyAndLeaveScalarOptionalsNull() {
        StructureItem item = new StructureItem(
            StructureKind.CLASS, null, null, SAMPLE_SPAN, null, null, null, null, null
        );

        assertEquals(List.of(), item.children());
        assertEquals(List.of(), item.decorators());
        assertNull(item.name());
        assertNull(item.visibility());
        assertNull(item.docComment());
        assertNull(item.signature());
        assertNull(item.bodySpan());
    }

    @Test
    void shouldSupportNestedChildrenForNamespacedStructures() {
        StructureItem method = new StructureItem(
            StructureKind.METHOD, "greet", null, SAMPLE_SPAN, null, null, null, null, null
        );
        StructureItem clazz = new StructureItem(
            StructureKind.CLASS, "Greeter", null, SAMPLE_SPAN, List.of(method), null, null, null, null
        );

        assertEquals(1, clazz.children().size());
        assertEquals("greet", clazz.children().get(0).name());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        StructureItem built = StructureItem.builder()
            .withKind(StructureKind.TRAIT)
            .withName("Comparable")
            .withSpan(SAMPLE_SPAN)
            .build();

        assertEquals(
            new StructureItem(StructureKind.TRAIT, "Comparable", null, SAMPLE_SPAN, null, null, null, null, null),
            built
        );
    }

    @Test
    void shouldRoundTripThroughJsonWithNestedChildrenAndDocComment() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        StructureItem item = new StructureItem(
            StructureKind.STRUCT, "Point", "pub", SAMPLE_SPAN, null, null, "a 2D point", "struct Point", null
        );

        String json = mapper.writeValueAsString(item);
        StructureItem parsed = mapper.readValue(json, StructureItem.class);

        assertEquals(item, parsed);
        assertEquals("Struct", parsed.kind().toString());
    }
}
