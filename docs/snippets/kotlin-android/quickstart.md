```kotlin title="Kotlin (Android)"
import dev.kreuzberg.tslp.android.TreeSitterLanguagePack
import dev.kreuzberg.tslp.android.PackConfig
import java.nio.file.Paths

suspend fun main() {
    val config = PackConfig(
        cacheDir = Paths.get("/data/data/com.example.app/cache/parsers"),
        languages = listOf("kotlin"),
        groups = null
    )
    TreeSitterLanguagePack.init(config)

    val lang = TreeSitterLanguagePack.getLanguage("kotlin")
    val parser = TreeSitterLanguagePack.getParser("kotlin")
    println("Root node kind: ${parser.parse("fun hello() {}").rootNode().type}")
}
```
