defmodule E2e.IncrementalParsingTest do
  use ExUnit.Case, async: true

  alias TreeSitterLanguagePack, as: Pack
  alias TreeSitterLanguagePack.{InputEdit, Parser, Point, Tree}

  test "reuses edited trees through both public parser interfaces" do
    {:ok, parser} = Pack.get_parser("python")
    original = Parser.parse(parser, "x = 'é'\n")

    {:ok, edited} =
      Tree.edited(original, %InputEdit{
        start_byte: 5,
        old_end_byte: 7,
        new_end_byte: 9,
        start_position: %Point{row: 0, column: 5},
        old_end_position: %Point{row: 0, column: 7},
        new_end_position: %Point{row: 0, column: 9}
      })

    source = "x = '🙂'\n"
    expected = parser |> Pack.parser_parse(source) |> sexp()
    assert parser |> Parser.parse_incremental(source, edited) |> sexp() == expected
    assert parser |> Pack.parser_parse_incremental(source, edited) |> sexp() == expected
  end

  defp sexp(tree), do: tree |> Pack.tree_root_node() |> Pack.node_to_sexp()
end
