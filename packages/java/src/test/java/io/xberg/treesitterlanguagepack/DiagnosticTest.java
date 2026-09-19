package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

class DiagnosticTest {

    private static final Span SAMPLE_SPAN = new Span(0, 1, 0, 0, 0, 1);

    @Test
    void shouldExposeMessageSeverityAndSpanAccessors() {
        Diagnostic diagnostic = new Diagnostic("unexpected token", DiagnosticSeverity.ERROR, SAMPLE_SPAN);

        assertEquals("unexpected token", diagnostic.message());
        assertEquals(DiagnosticSeverity.ERROR, diagnostic.severity());
        assertEquals(SAMPLE_SPAN, diagnostic.span());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        Diagnostic built = Diagnostic.builder()
            .withMessage("missing semicolon")
            .withSeverity(DiagnosticSeverity.WARNING)
            .withSpan(SAMPLE_SPAN)
            .build();

        assertEquals(new Diagnostic("missing semicolon", DiagnosticSeverity.WARNING, SAMPLE_SPAN), built);
    }

    @Test
    void shouldRoundTripThroughJsonIncludingNestedSeverityAndSpan() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        Diagnostic diagnostic = new Diagnostic("bad syntax", DiagnosticSeverity.INFO, SAMPLE_SPAN);

        String json = mapper.writeValueAsString(diagnostic);
        Diagnostic parsed = mapper.readValue(json, Diagnostic.class);

        assertEquals(diagnostic, parsed);
        assertEquals("Info", parsed.severity().toString());
    }
}
