```ruby title="Ruby"
require "tree_sitter_language_pack"

config = TreeSitterLanguagePack::PackConfig.new(languages: ["ruby"])
TreeSitterLanguagePack.init(config)

puts "Ruby available: #{TreeSitterLanguagePack.has_language("ruby")}"
puts "Languages: #{TreeSitterLanguagePack.language_count}"
```
