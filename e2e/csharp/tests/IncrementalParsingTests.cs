using Xunit;

namespace TreeSitterLanguagePack;

public class IncrementalParsingTests {
    [Fact]
    public void ReusesEditedTreeAcrossParses() {
        using var parser = TreeSitterLanguagePackConverter.GetParser("python");
        using var original = Assert.IsType<Tree>(parser.Parse("x = 1\n"));
        using var edited = original.Edited(new InputEdit {
            StartByte = 4,
            OldEndByte = 5,
            NewEndByte = 6,
            StartPosition = new Point { Column = 4 },
            OldEndPosition = new Point { Column = 5 },
            NewEndPosition = new Point { Column = 6 },
        });

        const string source = "x = 12\n";
        using var fresh = Assert.IsType<Tree>(parser.Parse(source));
        using var expected = fresh.RootNode();
        for (var run = 0; run < 2; run++) {
            using var incremental = Assert.IsType<Tree>(parser.ParseIncremental(source, edited));
            using var actual = incremental.RootNode();
            Assert.Equal(expected.ToSexp(), actual.ToSexp());
        }
    }
}
