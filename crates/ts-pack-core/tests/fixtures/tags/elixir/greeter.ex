defmodule Greeter.Behaviour do
  @callback greet(String.t()) :: String.t()
  @macrocallback trace(Macro.t()) :: Macro.t()
end

defprotocol Greeter.Size do
  def size(data)
end

defmodule Greeter.Person do
  defstruct [:name, age: 0]
end

defmodule Greeter.Error do
  defexception message: "greeting failed"
end

defimpl Greeter.Size, for: Greeter.Person do
  def size(person), do: String.length(person.name)
end

defmodule Greeter do
  @behaviour Greeter.Behaviour
  @default_name "world"

  @type name :: String.t()
  @typep count(item) :: {item, non_neg_integer()}
  @opaque token :: reference()

  @spec hello(name()) :: String.t()
  def hello(name) do
    name
    |> String.upcase()
    |> decorate()
  end

  def shout(name) do
    name |> String.upcase() |> decorate
  end

  def greet(name), do: hello(name)

  def factorial(0), do: 1
  def factorial(n) when n > 0, do: n * factorial(n - 1)

  defp decorate(text), do: "Hello " <> text

  defp secret, do: @default_name

  defmacro debug(expr) do
    quote do: IO.inspect(unquote(expr))
  end

  defmacro trace(expr), do: expr

  defguard is_short(text) when byte_size(text) < 5

  defdelegate length(text), to: String
end
