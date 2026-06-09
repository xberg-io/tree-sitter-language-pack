```elixir title="Elixir"
{:ok, nil} = TreeSitterLanguagePack.init(~s({"languages": ["elixir"]}))

count = TreeSitterLanguagePack.language_count()
IO.puts("Languages: #{count}")

IO.puts("Elixir available: #{TreeSitterLanguagePack.has_language("elixir")}")
```
