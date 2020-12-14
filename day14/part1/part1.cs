using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

struct Command
{
    public ulong address { get; set; }
    public ulong value { get; set; }

    public override string ToString() => $"({address}, {value})";
}

struct Instructions
{
    public string mask { get; set; }
    public List<Command> commands { get; set; }
}

class Part1
{
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
                    instructions.Last().commands.Add(new Command { address = address, value = value });
                }
            }
            return instructions;
        }
    }

    static ulong run(List<Instructions> program)
    {
        var memory = new Dictionary<ulong, ulong>();
        foreach (var instruction in program)
        {
            var mask0 = Convert.ToUInt64(instruction.mask.Replace("X", "0"), 2);
            var mask1 = Convert.ToUInt64(instruction.mask.Replace("X", "1"), 2);
            foreach (var command in instruction.commands)
            {
                memory[command.address] = (command.value | mask0) & mask1;
            }
        }

        return memory.Values.Aggregate((acc, val) => acc + val);
    }

    static void Main(string[] args)
    {
        var input = ReadFile(args[0]);
        var result = run(input);

        Console.WriteLine(result);
    }
}
