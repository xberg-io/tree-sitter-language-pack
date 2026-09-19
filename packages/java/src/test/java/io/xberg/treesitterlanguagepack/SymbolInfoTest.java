package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

class SymbolInfoTest {

    private static final Span SAMPLE_SPAN = new Span(0, 5, 0, 0, 0, 5);

    @Test
    void shouldExposeAllAccessors() {
        SymbolInfo symbol = new SymbolInfo("count", SymbolKind.VARIABLE, SAMPLE_SPAN, "int", "a running total");

        assertEquals("count", symbol.name());
        assertEquals(SymbolKind.VARIABLE, symbol.kind());
        assertEquals(SAMPLE_SPAN, symbol.span());
        assertEquals("int", symbol.typeAnnotation());
        assertEquals("a running total", symbol.doc());
    }

    @Test
    void shouldAllowNullOptionalFields() {
        SymbolInfo symbol = new SymbolInfo("Foo", SymbolKind.CLASS, SAMPLE_SPAN, null, null);

        assertNull(symbol.typeAnnotation());
        assertNull(symbol.doc());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        SymbolInfo built = SymbolInfo.builder()
            .withName("MAX")
            .withKind(SymbolKind.CONSTANT)
            .withSpan(SAMPLE_SPAN)
            .build();

        assertEquals(new SymbolInfo("MAX", SymbolKind.CONSTANT, SAMPLE_SPAN, null, null), built);
    }

    @Test
    void shouldRoundTripThroughJsonUsingSnakeCaseTypeAnnotationKey() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        SymbolInfo symbol = new SymbolInfo("x", SymbolKind.VARIABLE, SAMPLE_SPAN, "float", null);

        String json = mapper.writeValueAsString(symbol);

        assertTrue(json.contains("\"type_annotation\":\"float\""));
        assertEquals(symbol, mapper.readValue(json, SymbolInfo.class));
    }
}
