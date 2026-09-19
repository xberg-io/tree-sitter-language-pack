package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;

class DocstringFormatTest {

    @Test
    void shouldExposeSixVariants() {
        assertEquals(6, DocstringFormat.values().length);
    }

    @Test
    void shouldReturnWireFormatValueFromGetValue() {
        assertEquals("PythonTripleQuote", DocstringFormat.PYTHON_TRIPLE_QUOTE.getValue());
        assertEquals("JSDoc", DocstringFormat.JS_DOC.getValue());
        assertEquals("Rustdoc", DocstringFormat.RUSTDOC.getValue());
        assertEquals("GoDoc", DocstringFormat.GO_DOC.getValue());
        assertEquals("JavaDoc", DocstringFormat.JAVA_DOC.getValue());
        assertEquals("Other", DocstringFormat.OTHER.getValue());
    }

    @Test
    void shouldResolveFromValueCaseInsensitively() {
        assertEquals(DocstringFormat.JS_DOC, DocstringFormat.fromValue("jsdoc"));
    }

    @Test
    void shouldThrowIllegalArgumentExceptionForUnknownValue() {
        assertThrows(IllegalArgumentException.class, () -> DocstringFormat.fromValue("Doxygen"));
    }
}
