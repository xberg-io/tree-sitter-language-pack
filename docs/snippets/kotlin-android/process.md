```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.TreeSitterLanguagePack
import io.xberg.tslp.android.ProcessConfig

suspend fun analyzeCode(source: String) {
    val config = ProcessConfig(
        language = "kotlin",
        structure = true,
        imports = true,
        exports = false,
        comments = false,
        docstrings = false,
        symbols = false,
        diagnostics = false,
        chunks = null
    )

    val result = TreeSitterLanguagePack.processAsync(source, config)

    println("Language: ${result.language}")
    println("Detected ${result.structure.size} structural items")

    for (item in result.structure) {
        println("${item.kind}: ${item.name}")
        for (child in item.children) {
            println("  └ ${child.kind}: ${child.name}")
        }
    }
}
```
