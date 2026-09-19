package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;

import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.List;
import org.junit.jupiter.api.Test;

class DocstringInfoTest {

    private static final Span SAMPLE_SPAN = new Span(0, 10, 0, 0, 1, 0);

    @Test
    void shouldExposeAllAccessors() {
        List<DocSection> sections = List.of(new DocSection("Args", "x", "an integer"));
        DocstringInfo docstring = new DocstringInfo(
            "\"\"\"docs\"\"\"", DocstringFormat.PYTHON_TRIPLE_QUOTE, SAMPLE_SPAN, "my_func", sections
        );

        assertEquals("\"\"\"docs\"\"\"", docstring.text());
        assertEquals(DocstringFormat.PYTHON_TRIPLE_QUOTE, docstring.format());
        assertEquals(SAMPLE_SPAN, docstring.span());
        assertEquals("my_func", docstring.associatedItem());
        assertEquals(sections, docstring.parsedSections());
    }

    @Test
    void shouldNormalizeNullParsedSectionsToEmptyAndLeaveAssociatedItemNull() {
        DocstringInfo docstring = new DocstringInfo(
            "/** jsdoc */", DocstringFormat.JS_DOC, SAMPLE_SPAN, null, null
        );

        assertEquals(List.of(), docstring.parsedSections());
        assertNull(docstring.associatedItem());
    }

    @Test
    void shouldRoundTripThroughJsonWithNestedFormatEnumAndSections() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        DocstringInfo docstring = new DocstringInfo(
            "/// rustdoc", DocstringFormat.RUSTDOC, SAMPLE_SPAN, "my_fn",
            List.of(new DocSection("Returns", null, "an i32"))
        );

        String json = mapper.writeValueAsString(docstring);
        DocstringInfo parsed = mapper.readValue(json, DocstringInfo.class);

        assertEquals(docstring, parsed);
        assertEquals("Rustdoc", parsed.format().toString());
    }
}
