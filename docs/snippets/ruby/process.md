```ruby title="Ruby"
require "tree_sitter_language_pack"

config = TreeSitterLanguagePack::ProcessConfig.new(
  language: "ruby",
  structure: true,
  imports: true,
)

result = TreeSitterLanguagePack.process(
  "require 'json'\ndef parse(data)\n  JSON.parse(data)\nend",
  config
)

puts "Language: #{result.language}"
if result.structure
  result.structure.each do |item|
    puts "#{item.kind}: #{item.name}"
  end
end

if result.imports
  result.imports.each do |imp|
    puts "import: #{imp.source}"
  end
end
```
