```java title="Java"
import io.xberg.treesitterlanguagepack.PackConfig;
import io.xberg.treesitterlanguagepack.TreeSitterLanguagePack;

import java.util.List;
import java.util.Optional;

class Main {
    public static void main(String[] args) throws Exception {
        PackConfig config = PackConfig.builder()
                .withLanguages(Optional.of(List.of("java")))
                .build();
        TreeSitterLanguagePack.init(config);

        System.out.println("Java available: " + TreeSitterLanguagePack.hasLanguage("java"));
        System.out.println("Languages: " + TreeSitterLanguagePack.languageCount());
    }
}
```
