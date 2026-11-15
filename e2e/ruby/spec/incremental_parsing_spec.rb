# frozen_string_literal: true

require "tree_sitter_language_pack"

RSpec.describe TreeSitterLanguagePack::Parser do
  it "reuses the same edited tree across parses" do
    parser = TreeSitterLanguagePack.get_parser("python")
    original = parser.parse("x = 'é'\n")
    point = -> (column) { TreeSitterLanguagePack::Point.new(row: 0, column:) }
    edited = original.edited(
      TreeSitterLanguagePack::InputEdit.new(
        start_byte: 5,
        old_end_byte: 7,
        new_end_byte: 9,
        start_position: point.call(5),
        old_end_position: point.call(7),
        new_end_position: point.call(9)
      )
    )

    source = "x = '🙂'\n"
    expected = parser.parse(source).root_node.to_sexp
    actual = Array.new(2) { parser.parse_incremental(source, edited).root_node.to_sexp }
    expect(actual).to(eq([expected, expected]))
  end
end
