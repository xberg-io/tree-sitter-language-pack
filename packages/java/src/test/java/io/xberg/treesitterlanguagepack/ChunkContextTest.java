package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.List;
import org.junit.jupiter.api.Test;

class ChunkContextTest {

    @Test
    void shouldExposeAllAccessors() {
        ChunkContext context = new ChunkContext(
            "python", 0, 3, List.of("function_definition"), List.of("Foo", "bar"),
            List.of("bar"), List.of(), List.of(), false
        );

        assertEquals("python", context.language());
        assertEquals(0, context.chunkIndex());
        assertEquals(3, context.totalChunks());
        assertEquals(List.of("function_definition"), context.nodeTypes());
        assertEquals(List.of("Foo", "bar"), context.contextPath());
        assertEquals(List.of("bar"), context.symbolsDefined());
        assertFalse(context.hasErrorNodes());
    }

    @Test
    void shouldDefaultBuilderListFieldsToEmptyWhenUnset() {
        ChunkContext built = ChunkContext.builder()
            .withLanguage("go")
            .withChunkIndex(1)
            .withTotalChunks(1)
            .withHasErrorNodes(true)
            .build();

        assertEquals(List.of(), built.nodeTypes());
        assertEquals(List.of(), built.contextPath());
        assertEquals(List.of(), built.symbolsDefined());
        assertEquals(List.of(), built.comments());
        assertEquals(List.of(), built.docstrings());
        assertTrue(built.hasErrorNodes());
    }

    @Test
    void shouldRoundTripThroughJsonWithNestedCommentsAndDocstrings() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        Span span = new Span(0, 2, 0, 0, 0, 2);
        ChunkContext context = new ChunkContext(
            "rust", 2, 5, List.of("struct_item"), List.of("Foo"), List.of("Foo"),
            List.of(new CommentInfo("// c", CommentKind.LINE, span, null)),
            List.of(new DocstringInfo("/// d", DocstringFormat.RUSTDOC, span, null, null)),
            true
        );

        String json = mapper.writeValueAsString(context);
        ChunkContext parsed = mapper.readValue(json, ChunkContext.class);

        assertEquals(context, parsed);
        assertTrue(json.contains("\"chunk_index\":2"));
    }
}
