package tspack

import "testing"

func TestIncrementalParseReusesEditedTree(t *testing.T) {
	const before = "x = 'é'\ny = 1\n"
	const after = "x = '🙂'\ny = 1\n"

	parser, err := GetParser("python")
	if err != nil {
		t.Fatal(err)
	}
	defer parser.Free()

	original := parser.Parse(before)
	if original == nil {
		t.Fatal("initial parse returned no tree")
	}
	defer original.Free()
	originalRoot := original.RootNode()
	defer originalRoot.Free()

	edited, err := original.Edited(InputEdit{
		StartByte: 5, OldEndByte: 7, NewEndByte: 9,
		StartPosition:  &Point{Row: 0, Column: 5},
		OldEndPosition: &Point{Row: 0, Column: 7},
		NewEndPosition: &Point{Row: 0, Column: 9},
	})
	if err != nil {
		t.Fatal(err)
	}
	defer edited.Free()

	incremental := parser.ParseIncremental(after, edited)
	if incremental == nil {
		t.Fatal("incremental parse returned no tree")
	}
	defer incremental.Free()
	root := incremental.RootNode()
	defer root.Free()
	if root.EndByte() != uint(len(after)) || originalRoot.EndByte() != uint(len(before)) {
		t.Fatalf("unexpected tree ends: original=%d incremental=%d", originalRoot.EndByte(), root.EndByte())
	}

	limit := uint(1)
	parser.SetMaxSourceBytes(&limit)
	if rejected := parser.ParseIncremental(after, edited); rejected != nil {
		rejected.Free()
		t.Fatal("over-limit incremental parse returned a tree")
	}
}
