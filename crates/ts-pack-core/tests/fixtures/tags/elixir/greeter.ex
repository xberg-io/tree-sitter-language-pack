defmodule Greeter do
  def hello(name) do
    String.upcase(name)
  end

  defp secret, do: :ok

  defmacro debug(expr) do
    quote do: IO.inspect(unquote(expr))
  end
end
