```csharp title="C#"
using TreeSitterLanguagePack;

var dm = DownloadManager.New("1.9.0");

dm.DownloadAllBestEffort();

var downloaded = dm.InstalledLanguages();
Console.WriteLine($"Downloaded languages: {string.Join(", ", downloaded)}");

var registry = LanguageRegistry.Default();
var available = registry.AvailableLanguages();
Console.WriteLine($"Total available: {available.Count}");

dm.Dispose();
registry.Dispose();
```
