```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\PackConfig;
use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;

$config = new PackConfig(
    cache_dir: null,
    languages: ["php", "javascript"],
    groups: null,
);
TreeSitterLanguagePack::init($config);

$count = TreeSitterLanguagePack::download(["python", "rust"]);
echo "Ensured {$count} languages\n";

foreach (TreeSitterLanguagePack::downloadedLanguages() as $name) {
    echo "cached: {$name}\n";
}
```
