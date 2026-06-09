```ruby title="Ruby"
require "tree_sitter_language_pack"

config = TreeSitterLanguagePack::PackConfig.new(languages: ["ruby", "python"])
TreeSitterLanguagePack.init(config)

count = TreeSitterLanguagePack.download(["rust", "javascript"])
puts "Ensured #{count} languages"

TreeSitterLanguagePack.downloaded_languages.each do |name|
  puts "cached: #{name}"
end
```
