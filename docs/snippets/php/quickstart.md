```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;

if (TreeSitterLanguagePack::hasLanguage("php")) {
    echo "PHP grammar is available\n";
}

echo "Total languages: " . TreeSitterLanguagePack::languageCount() . "\n";
```
