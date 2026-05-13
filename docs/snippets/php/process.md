```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;

$config = new ProcessConfig(
    language: "php",
    structure: true,
    imports: true,
    exports: true,
    comments: false,
    docstrings: false,
    symbols: false,
    diagnostics: false,
    chunk_max_size: null,
);

$result = TreeSitterLanguagePack::process(
    "<?php namespace App; class Controller { public function index() {} }",
    $config,
);

echo "Language: " . $result->language . "\n";
foreach ($result->getStructure() as $item) {
    echo $item->kind . ": " . ($item->name ?? "(anonymous)") . "\n";
}
```
