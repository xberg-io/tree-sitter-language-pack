```elixir title="Elixir"
{:ok, json} =
  TreeSitterLanguagePack.process(
    "defmodule MyApp do\n  def hello, do: :world\nend",
    ~s({"language": "elixir", "structure": true, "imports": true})
  )

result = Jason.decode!(json)
IO.puts("Language: #{result["language"]}")

for item <- result["structure"] do
  IO.puts("#{item["kind"]}: #{item["name"]}")
end
```
