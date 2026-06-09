```java title="Java"
import dev.kreuzberg.treesitterlanguagepack.ProcessConfig;
import dev.kreuzberg.treesitterlanguagepack.ProcessResult;
import dev.kreuzberg.treesitterlanguagepack.StructureItem;
import dev.kreuzberg.treesitterlanguagepack.TreeSitterLanguagePack;

class Main {
    public static void main(String[] args) throws Exception {
        ProcessConfig config = ProcessConfig.builder()
                .withLanguage("java")
                .withStructure(true)
                .withImports(true)
                .build();

        ProcessResult result = TreeSitterLanguagePack.process(
                "import java.util.List;\npublic class App { public void run() {} }",
                config);

        System.out.println("Language: " + result.language());
        for (StructureItem item : result.structure()) {
            System.out.println(item.kind() + ": " + item.name());
        }
    }
}
```
