defmodule Part1 do
  defp load(filepath) do
    {:ok, contents} = File.read(filepath)
    contents |> String.split("\n", trim: true)
  end

  defp is_valid(line) do
    [policy, letter, password] = String.split(line, " ", trim: true)
    [min, max] = policy |> String.split("-") |> Enum.map(&String.to_integer/1)
    key = letter |> String.first()
    occurences = password |> String.graphemes() |> Enum.count(fn letter -> letter == key end)
    min <= occurences and occurences <= max
  end

  defp parse(input) do
    input |> Enum.map(&is_valid/1) |> Enum.count(fn elem -> elem == true end)
  end

  def main(args) do
    input = load(args)
    IO.puts(parse(input))
  end
end
