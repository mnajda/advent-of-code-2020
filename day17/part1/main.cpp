#include <array>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <unordered_set>
#include <vector>

struct Point
{
    int x;
    int y;
    int z;

    bool operator==(const Point& other) const
    {
        return (x == other.x) and (y == other.y) and (z == other.z);
    }

    Point operator+(const Point& other) const
    {
        return {x + other.x, y + other.y, z + other.z};
    }
};

namespace std
{
template<> struct hash<Point>
{
    size_t operator()(const Point& point) const
    {   
        return std::hash<int>{}(point.x) ^ std::hash<int>{}(point.y) ^ std::hash<int>{}(point.z);
    }
};
} // namespace std

namespace
{
constexpr auto make_adjacent()
{
    auto iter = 0;
    std::array<Point, 26> adjacent{};

    for (auto i = -1; i < 2; ++i)
    {
        for (auto j = -1; j < 2; ++j)
        {
            for (auto k = -1; k < 2; ++k)
            {
                if (not (i == 0 and j == 0 and k == 0))
                {
                    adjacent[iter++] = Point{i, j, k};
                }
            }
        }
    }

    return adjacent;
}

constexpr auto adjacent_points = make_adjacent();
} // namespace

std::vector<std::vector<char>> load_file(const char* filepath)
{
    std::vector<std::vector<char>> output{};
    std::ifstream file(filepath);

    std::string input{};
    while (std::getline(file, input))
    {
        const std::vector<char> line(input.begin(), input.end());
        output.emplace_back(line);
    }

    return output;
}

std::unordered_set<Point> create_inital_state(std::vector<std::vector<char>> input)
{
    std::unordered_set<Point> cubes{};

    for (auto y = 0; y < input.size(); ++y)
    {
        for (auto x = 0; x < input[y].size(); ++x)
        {
            if (input[y][x] == '#')
            {
                cubes.insert({x, y, 0});
            }
        }
    }

    return cubes;
}

int count_active_neighbours(const Point& cube, const std::unordered_set<Point>& state)
{
    auto result = 0;
    for (const auto& point : adjacent_points)
    {
        const auto neighbour = cube + point;
        if (state.find(neighbour) != state.end())
        {
            ++result;
        }
    }
    return result;
}

std::unordered_set<Point> simulate(std::unordered_set<Point> state)
{
    std::unordered_set<Point> new_state{};

    for (const auto& point : adjacent_points)
    {
        for (const auto& cube : state)
        {
            const auto neighbour = cube + point;
            const auto active_neighbours = count_active_neighbours(neighbour, state);

            if (state.find(neighbour) != state.end())
            {
                if ((active_neighbours == 2) or (active_neighbours == 3))
                {
                    new_state.insert(neighbour);
                }
            }
            else
            {
                if (active_neighbours == 3)
                {
                    new_state.insert(neighbour);
                }
            }
        }
    }

    return new_state;
}

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        throw std::runtime_error("No filepath given");
    }
    const auto input = load_file(argv[1]);
    
    auto state = create_inital_state(std::move(input));
    for (auto cycle = 0; cycle < 6; ++cycle)
    {
        state = simulate(std::move(state));
    }

    std::cout << state.size() << std::endl;
}
