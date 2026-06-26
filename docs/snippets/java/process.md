```java title="Java"
import io.xberg.treesitterlanguagepack.ProcessConfig;
import io.xberg.treesitterlanguagepack.ProcessResult;
import io.xberg.treesitterlanguagepack.StructureItem;
import io.xberg.treesitterlanguagepack.TreeSitterLanguagePack;

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
