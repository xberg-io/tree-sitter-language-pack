package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;

import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.List;
import org.junit.jupiter.api.Test;

class DataNodeTest {

    private static final Span SAMPLE_SPAN = new Span(0, 4, 0, 0, 0, 4);

    @Test
    void shouldExposeAllAccessorsForKeyValueNode() {
        DataNode node = new DataNode(DataNodeKind.KEY_VALUE, "name", "value", null, null, SAMPLE_SPAN);

        assertEquals(DataNodeKind.KEY_VALUE, node.kind());
        assertEquals("name", node.key());
        assertEquals("value", node.value());
        assertEquals(List.of(), node.attributes());
        assertEquals(List.of(), node.children());
        assertEquals(SAMPLE_SPAN, node.span());
    }

    @Test
    void shouldSupportNestedElementNodeWithAttributesAndChildren() {
        DataAttribute attribute = new DataAttribute("id", "1", SAMPLE_SPAN);
        DataNode child = new DataNode(DataNodeKind.SEQUENCE, "0", "item", null, null, SAMPLE_SPAN);
        DataNode parent = new DataNode(
            DataNodeKind.ELEMENT, "ul", null, List.of(attribute), List.of(child), SAMPLE_SPAN
        );

        assertEquals(List.of(attribute), parent.attributes());
        assertEquals(List.of(child), parent.children());
        assertEquals("0", parent.children().get(0).key());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        DataNode built = DataNode.builder()
            .withKind(DataNodeKind.KEY_VALUE)
            .withKey("k")
            .withValue("v")
            .withSpan(SAMPLE_SPAN)
            .build();

        assertEquals(new DataNode(DataNodeKind.KEY_VALUE, "k", "v", null, null, SAMPLE_SPAN), built);
    }

    @Test
    void shouldRoundTripThroughJsonWithRecursiveChildren() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        DataNode child = new DataNode(DataNodeKind.KEY_VALUE, "leaf", "1", null, null, SAMPLE_SPAN);
        DataNode root = new DataNode(DataNodeKind.ELEMENT, "root", null, null, List.of(child), SAMPLE_SPAN);

        String json = mapper.writeValueAsString(root);
        DataNode parsed = mapper.readValue(json, DataNode.class);

        assertEquals(root, parsed);
        assertEquals("leaf", parsed.children().get(0).key());
    }
}
