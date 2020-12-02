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

    for (auto i = 0; i < input.size(); ++i)
    {
        const auto first = target - input[i];
        for (auto k = i + 1; k < input.size(); ++k)
        {
            const auto result = first - input[k];
            if (set.find(result) != set.end())
            {
                return input[i] * input[k] * result;
            }
            set.insert(input[k]);
        }
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
