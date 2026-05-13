```elixir title="Elixir"
{:ok, nil} = TreeSitterLanguagePack.init(~s({"languages": ["elixir", "erlang"]}))

{:ok, count} = TreeSitterLanguagePack.download(["python", "rust"])
IO.puts("Ensured #{count} languages")

langs = TreeSitterLanguagePack.downloaded_languages()
IO.inspect(langs, label: "cached")
```
