#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <optional>
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

std::optional<std::int64_t> pick_available_adapter(
    const std::int64_t jolts,
    const std::vector<int64_t>::const_iterator begin,
    const std::vector<int64_t>& input)
{
    const auto available = std::find_if(
        begin,
        input.end(),
        [jolts](const auto adapter) {
            return adapter - jolts <= 3;
    });

    if (available != input.end())
    {
        return *available;
    }
    return std::nullopt;
}

std::int64_t solve(std::vector<int64_t> input)
{
    auto one_jolt_differences{0};
    auto three_jolt_differences{0};
    
    input.emplace_back(0);
    std::sort(input.begin(), input.end());
    for (auto adapter = input.begin(); adapter != input.end(); ++adapter)
    {
        const auto available = pick_available_adapter(*adapter, adapter + 1, input);
        if (available)
        {
            const auto difference = *available - *adapter;
            switch (difference)
            {
                case 1:
                    ++one_jolt_differences;
                    break;
                case 3:
                    ++three_jolt_differences;
                    break;
                default:
                    break;
            }
        }
    }
    ++three_jolt_differences;

    return one_jolt_differences * three_jolt_differences;
}

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        throw std::runtime_error("No filepath given");
    }

    auto input = load_file(argv[1]);
    const auto result = solve(std::move(input));

    std::cout << result << std::endl;
}
