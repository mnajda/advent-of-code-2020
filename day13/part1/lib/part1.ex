defmodule Part1 do
  defp load(filepath) do
    {:ok, contents} = File.read(filepath)
    [head, tail] = contents |> String.split("\n", trim: true)

    {head |> String.to_integer(),
     tail
     |> String.split(",", trim: true)
     |> Enum.filter(fn val -> val != "x" end)
     |> Enum.map(&String.to_integer/1)}
  end

  defp find_bus(timestamp, buses) do
    {delay, id} = buses |> Enum.map(fn bus -> {bus - rem(timestamp, bus), bus} end) |> Enum.min()
    delay * id
  end

  def main(args) do
    {timestamp, buses} = load(args)
    IO.puts(find_bus(timestamp, buses))
  end
end
