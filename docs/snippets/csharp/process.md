```csharp title="C#"
using TreeSitterLanguagePack;

var registry = LanguageRegistry.Default();

var config = new ProcessConfig
{
    Language = "csharp",
    Structure = true,
    Imports = true
};

var result = registry.Process("public class Greeter { }", config);

foreach (var item in result.Structure)
{
    Console.WriteLine($"Kind: {item.Kind}, Name: {item.Name}");
}

registry.Dispose();
```
