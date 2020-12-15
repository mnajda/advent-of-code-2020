using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

struct Command
{
    public List<ulong> addresses { get; set; }
    public ulong value { get; set; }
}

struct Instructions
{
    public string mask { get; set; }
    public List<Command> commands { get; set; }
}

class Part1
{
    static void Generate(ulong address, int iter, List<int> indexes, List<ulong> addresses)
    {
        if (iter == indexes.Count())
        {
            return;
        }

        var index = indexes[iter];
        var addr0 = address & ~(1UL << index);
        var addr1 = address | (1UL << index);

        addresses.Add(addr0);
        addresses.Add(addr1);
        Generate(addr0, iter + 1, indexes, addresses);
        Generate(addr1, iter + 1, indexes, addresses);
    }

    static List<ulong> GenerateAddresses(ulong address, string mask)
    {
        var addresses = new List<ulong>();
        var addr = address | Convert.ToUInt64(mask.Replace("X", "0"), 2);
        var indexes = new List<int>();

        for (var index = mask.Length; index > 0; index--)
        {
            if (mask[index - 1] == 'X')
            {
                indexes.Add(mask.Length - index);
            }
        }

        Generate(addr, 0, indexes, addresses);

        return addresses;
    }

    static List<Instructions> ReadFile(string path)
    {
        using (var reader = new StreamReader(path))
        {
            var contents = reader.ReadToEnd().Split("\n");
            var addressRegex = new Regex(@"\d+");
            var instructions = new List<Instructions>();

            foreach (var line in contents)
            {
                var input = line.Split('=').Select(p => p.Trim()).ToList();
                if (input[0] == "mask")
                {
                    instructions.Add(new Instructions { mask = input[1], commands = new List<Command>() });
                }
                else
                {
                    var address = Convert.ToUInt64(addressRegex.Match(input[0]).Value);
                    var value = Convert.ToUInt64(input[1]);

                    var addresses = GenerateAddresses(address, instructions.Last().mask);
                    
                    instructions.Last().commands.Add(new Command { addresses = addresses, value = value });
                }
            }
            return instructions;
        }
    }

    static ulong Run(List<Instructions> program)
    {
        var memory = new Dictionary<ulong, ulong>();
        foreach (var instruction in program)
        {
            foreach (var command in instruction.commands)
            {
                foreach (var address in command.addresses)
                {
                    memory[address] = command.value;
                }
            }
        }

        return memory.Values.Aggregate((acc, val) => acc + val);
    }

    static void Main(string[] args)
    {
        var input = ReadFile(args[0]);
        var result = Run(input);

        Console.WriteLine(result);
    }
}
