defmodule Part2 do
  defp load(filepath) do
    {:ok, contents} = File.read(filepath)
    contents |> String.split("\n", trim: true)
  end

  defp is_valid(line) do
    [policy, letter, password] = String.split(line, " ", trim: true)

    [first_pos, last_pos] =
      policy
      |> String.split("-")
      |> Enum.map(&String.to_integer/1)
      |> Enum.map(fn elem -> elem - 1 end)

    [{first, _}, {last, _}] =
      password
      |> String.graphemes()
      |> Enum.with_index()
      |> Enum.filter(fn {_, iter} -> iter == first_pos or iter == last_pos end)

    key = letter |> String.first()
    (first == key and last != key) or (first != key and last == key)
  end

  defp parse(input) do
    input |> Enum.map(&is_valid/1) |> Enum.count(fn elem -> elem == true end)
  end

  def main(args) do
    input = load(args)
    IO.puts(parse(input))
  end
end
