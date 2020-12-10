#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <optional>
#include <unordered_map>
#include <vector>

std::vector<std::int64_t> load_file(const char* filepath)
{
    std::vector<std::int64_t> output{};
    std::ifstream file(filepath);

    std::int64_t input{};
    while (file >> input)
    {
        output.emplace_back(input);
    }

    return output;
}

std::int64_t solve(std::vector<int64_t> input)
{
    std::unordered_map<std::int64_t, std::int64_t> possibilities{{0, 1}};
    for (const auto value : input)
    {
        const auto count = possibilities[value - 1] + possibilities[value - 2] + possibilities[value - 3];
        possibilities[value] += count;
    }

    return possibilities[input.back()];
}

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        throw std::runtime_error("No filepath given");
    }

    auto input = load_file(argv[1]);

    input.emplace_back(0);
    std::sort(input.begin(), input.end());

    const auto result = solve(std::move(input));

    std::cout << result << std::endl;
}
