#include <cstdint>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <vector>
#include <unordered_set>

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

std::int64_t calculate(const std::vector<std::int64_t> input)
{
    constexpr auto target{2020};
    std::unordered_set<std::int64_t> set{};

    for (const auto value : input)
    {
        const auto result = target - value;
        if (set.find(result) != set.end())
        {
            return result * value;
        }
        set.insert(value);
    }

    throw std::runtime_error("No such values that sum up to 2020");
}

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        throw std::runtime_error("No filepath given");
    }
    const auto input = load_file(argv[1]);
    std::cout << calculate(std::move(input)) << std::endl;
}
