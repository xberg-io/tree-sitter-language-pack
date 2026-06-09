```csharp title="C#"
using TreeSitterLanguagePack;

var parser = Parser.Default();
parser.SetLanguage("python");

var tree = parser.Parse("def hello():\n    print('world')\n");
var root = tree!.RootNode();

Console.WriteLine($"Root kind: {root.Kind()}");

parser.Dispose();
```
