package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

class ExportInfoTest {

    private static final Span SAMPLE_SPAN = new Span(0, 6, 0, 0, 0, 6);

    @Test
    void shouldExposeNameKindAndSpanAccessors() {
        ExportInfo export = new ExportInfo("foo", ExportKind.NAMED, SAMPLE_SPAN);

        assertEquals("foo", export.name());
        assertEquals(ExportKind.NAMED, export.kind());
        assertEquals(SAMPLE_SPAN, export.span());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        ExportInfo built = ExportInfo.builder().withName("bar").withKind(ExportKind.DEFAULT).withSpan(SAMPLE_SPAN).build();

        assertEquals(new ExportInfo("bar", ExportKind.DEFAULT, SAMPLE_SPAN), built);
    }

    @Test
    void shouldRoundTripThroughJsonWithReExportKind() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        ExportInfo export = new ExportInfo("baz", ExportKind.RE_EXPORT, SAMPLE_SPAN);

        String json = mapper.writeValueAsString(export);
        ExportInfo parsed = mapper.readValue(json, ExportInfo.class);

        assertEquals(export, parsed);
        assertEquals("ReExport", parsed.kind().toString());
    }
}
