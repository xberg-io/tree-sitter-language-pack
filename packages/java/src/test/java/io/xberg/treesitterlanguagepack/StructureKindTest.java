package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

class StructureKindTest {

    private static final ObjectMapper MAPPER = new ObjectMapper();

    @Test
    void shouldExposeElevenVariants() {
        assertEquals(11, StructureKind.class.getPermittedSubclasses().length);
    }

    @Test
    void shouldReturnWireFormatValueFromSerialization() throws Exception {
        assertEquals("\"Function\"", MAPPER.writeValueAsString(new StructureKind.Function()));
        assertEquals("\"Method\"", MAPPER.writeValueAsString(new StructureKind.Method()));
        assertEquals("\"Class\"", MAPPER.writeValueAsString(new StructureKind.Class()));
        assertEquals("\"Struct\"", MAPPER.writeValueAsString(new StructureKind.Struct()));
        assertEquals("\"Interface\"", MAPPER.writeValueAsString(new StructureKind.Interface()));
        assertEquals("\"Enum\"", MAPPER.writeValueAsString(new StructureKind.Enum()));
        assertEquals("\"Module\"", MAPPER.writeValueAsString(new StructureKind.Module()));
        assertEquals("\"Trait\"", MAPPER.writeValueAsString(new StructureKind.Trait()));
        assertEquals("\"Impl\"", MAPPER.writeValueAsString(new StructureKind.Impl()));
        assertEquals("\"Namespace\"", MAPPER.writeValueAsString(new StructureKind.Namespace()));
        assertEquals("{\"Other\":\"macro\"}", MAPPER.writeValueAsString(new StructureKind.Other("macro")));
    }

    @Test
    void shouldResolveVariantFromWireValue() throws Exception {
        assertEquals(new StructureKind.Function(), MAPPER.readValue("\"Function\"", StructureKind.class));
        assertEquals(new StructureKind.Method(), MAPPER.readValue("\"Method\"", StructureKind.class));
        assertEquals(new StructureKind.Class(), MAPPER.readValue("\"Class\"", StructureKind.class));
        assertEquals(new StructureKind.Struct(), MAPPER.readValue("\"Struct\"", StructureKind.class));
        assertEquals(new StructureKind.Interface(), MAPPER.readValue("\"Interface\"", StructureKind.class));
        assertEquals(new StructureKind.Enum(), MAPPER.readValue("\"Enum\"", StructureKind.class));
        assertEquals(new StructureKind.Module(), MAPPER.readValue("\"Module\"", StructureKind.class));
        assertEquals(new StructureKind.Trait(), MAPPER.readValue("\"Trait\"", StructureKind.class));
        assertEquals(new StructureKind.Impl(), MAPPER.readValue("\"Impl\"", StructureKind.class));
        assertEquals(new StructureKind.Namespace(), MAPPER.readValue("\"Namespace\"", StructureKind.class));
        assertEquals(new StructureKind.Other("macro"), MAPPER.readValue("{\"Other\":\"macro\"}", StructureKind.class));
    }

    @Test
    void shouldThrowJsonProcessingExceptionForUnknownValue() {
        assertThrows(JsonProcessingException.class, () -> MAPPER.readValue("\"Macro\"", StructureKind.class));
    }
}
