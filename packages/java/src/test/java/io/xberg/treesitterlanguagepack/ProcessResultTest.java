package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;

import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.List;
import org.junit.jupiter.api.Test;

class ProcessResultTest {

    private static final FileMetrics SAMPLE_METRICS = new FileMetrics(1, 1, 0, 0, 10, 1, 0, 1);

    @Test
    void shouldExposeAllAccessors() {
        ProcessResult result = new ProcessResult(
            "python", SAMPLE_METRICS, List.of(), List.of(), List.of(), List.of(), List.of(),
            List.of(), List.of(), List.of(), null
        );

        assertEquals("python", result.language());
        assertEquals(SAMPLE_METRICS, result.metrics());
        assertEquals(List.of(), result.structure());
        assertEquals(List.of(), result.imports());
        assertEquals(List.of(), result.exports());
        assertNull(result.data());
    }

    @Test
    void shouldNormalizeEveryNullListFieldToAnEmptyList() {
        ProcessResult result = new ProcessResult(
            "go", SAMPLE_METRICS, null, null, null, null, null, null, null, null, null
        );

        assertEquals(List.of(), result.structure());
        assertEquals(List.of(), result.imports());
        assertEquals(List.of(), result.exports());
        assertEquals(List.of(), result.comments());
        assertEquals(List.of(), result.docstrings());
        assertEquals(List.of(), result.symbols());
        assertEquals(List.of(), result.diagnostics());
        assertEquals(List.of(), result.chunks());
        assertNull(result.data());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        ProcessResult built = ProcessResult.builder().withLanguage("java").withMetrics(SAMPLE_METRICS).build();

        assertEquals("java", built.language());
        assertEquals(SAMPLE_METRICS, built.metrics());
        assertEquals(List.of(), built.structure());
    }

    @Test
    void shouldRoundTripThroughJsonWithNestedStructureAndMetrics() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        Span span = new Span(0, 3, 0, 0, 0, 3);
        ProcessResult result = new ProcessResult(
            "ruby", SAMPLE_METRICS,
            List.of(new StructureItem(StructureKind.METHOD, "run", null, span, null, null, null, null, null)),
            null, null, null, null, null, null, null, null
        );

        String json = mapper.writeValueAsString(result);
        ProcessResult parsed = mapper.readValue(json, ProcessResult.class);

        assertEquals(result, parsed);
        assertEquals("run", parsed.structure().get(0).name());
    }
}
