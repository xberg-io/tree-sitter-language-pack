```java title="Java"
import io.xberg.treesitterlanguagepack.PackConfig;
import io.xberg.treesitterlanguagepack.TreeSitterLanguagePack;

import java.util.List;
import java.util.Optional;

class Main {
    public static void main(String[] args) throws Exception {
        PackConfig config = PackConfig.builder()
                .withLanguages(Optional.of(List.of("java", "kotlin")))
                .build();
        TreeSitterLanguagePack.init(config);

        long ensured = TreeSitterLanguagePack.download(List.of("python", "rust"));
        System.out.println("Ensured " + ensured + " languages");

        for (String name : TreeSitterLanguagePack.downloadedLanguages()) {
            System.out.println("cached: " + name);
        }
    }
}
```
