#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <vector>
#include <unordered_set>

std::vector<std::int64_t> load_file(const std::string& filepath)
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

    for (auto i = 0; i < input.size(); ++i)
    {
        for (auto j = i + 1; j < input.size(); ++j)
        {
            const auto result = target - input[i] - input[j];
            if ((std::find(input.begin(), input.end(), result)) != input.end())
            {
                return input[i] * input[j] * result;
            }
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
    const auto filepath{argv[1]};
    const auto input = load_file(filepath);
    std::cout << calculate(std::move(input)) << std::endl;
}
