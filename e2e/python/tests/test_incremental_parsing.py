from tree_sitter_language_pack import get_parser


def test_native_parser_reuses_edited_tree_with_utf8_coordinates() -> None:
    before = "x = 'é'\ny = 1\n".encode()
    after = "x = '🙂'\ny = 1\n".encode()
    parser = get_parser("python")
    original = parser.parse(before)
    edited = original.copy()
    edited.edit(
        start_byte=5,
        old_end_byte=7,
        new_end_byte=9,
        start_point=(0, 5),
        old_end_point=(0, 7),
        new_end_point=(0, 9),
    )

    incremental = parser.parse(after, edited)
    fresh = parser.parse(after)
    assert str(incremental.root_node) == str(fresh.root_node)
    assert incremental.root_node.end_byte == len(after)
    assert original.root_node.end_byte == len(before)
