```kotlin title="Kotlin (Android)"
import dev.kreuzberg.tslp.android.TreeSitterLanguagePack
import dev.kreuzberg.tslp.android.PackConfig
import android.app.Application
import java.nio.file.Paths

class MyApp : Application() {
    override fun onCreate() {
        super.onCreate()

        val cacheDir = Paths.get(cacheDir.absolutePath, "tree-sitter")
        val config = PackConfig(
            cacheDir = cacheDir,
            languages = listOf("kotlin", "java", "xml"),
            groups = null
        )
        TreeSitterLanguagePack.init(config)

        val downloaded = TreeSitterLanguagePack.downloadedLanguages()
        println("Downloaded parsers: $downloaded")

        val count = TreeSitterLanguagePack.languageCount()
        println("Total available languages: $count")
    }
}
```
