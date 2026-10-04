package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

class DocstringFormatTest {

    private static final ObjectMapper MAPPER = new ObjectMapper();

    @Test
    void shouldExposeSixVariants() {
        assertEquals(6, DocstringFormat.class.getPermittedSubclasses().length);
    }

    @Test
    void shouldReturnWireFormatValueFromSerialization() throws Exception {
        assertEquals("\"PythonTripleQuote\"", MAPPER.writeValueAsString(new DocstringFormat.PythonTripleQuote()));
        assertEquals("\"JSDoc\"", MAPPER.writeValueAsString(new DocstringFormat.JSDoc()));
        assertEquals("\"Rustdoc\"", MAPPER.writeValueAsString(new DocstringFormat.Rustdoc()));
        assertEquals("\"GoDoc\"", MAPPER.writeValueAsString(new DocstringFormat.GoDoc()));
        assertEquals("\"JavaDoc\"", MAPPER.writeValueAsString(new DocstringFormat.JavaDoc()));
        assertEquals("{\"Other\":\"rst\"}", MAPPER.writeValueAsString(new DocstringFormat.Other("rst")));
    }

    @Test
    void shouldResolveVariantFromWireValue() throws Exception {
        assertEquals(new DocstringFormat.PythonTripleQuote(), MAPPER.readValue("\"PythonTripleQuote\"", DocstringFormat.class));
        assertEquals(new DocstringFormat.JSDoc(), MAPPER.readValue("\"JSDoc\"", DocstringFormat.class));
        assertEquals(new DocstringFormat.Rustdoc(), MAPPER.readValue("\"Rustdoc\"", DocstringFormat.class));
        assertEquals(new DocstringFormat.GoDoc(), MAPPER.readValue("\"GoDoc\"", DocstringFormat.class));
        assertEquals(new DocstringFormat.JavaDoc(), MAPPER.readValue("\"JavaDoc\"", DocstringFormat.class));
        assertEquals(new DocstringFormat.Other("rst"), MAPPER.readValue("{\"Other\":\"rst\"}", DocstringFormat.class));
    }

    @Test
    void shouldThrowJsonProcessingExceptionForUnknownValue() {
        assertThrows(JsonProcessingException.class, () -> MAPPER.readValue("\"Doxygen\"", DocstringFormat.class));
    }
}
