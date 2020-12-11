STEPS = [(0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)]

function get_adjacent_occupied(seat, map)
    (y, x) = seat

    return filter(
        adjacent -> adjacent == '#',
        [map[y + dy][x + dx] for (dy, dx) in STEPS if 1 <= y + dy <= length(map) && 1 <= x + dx <= length(map[y])])
end

function simulate(input)
    old = input
    new = deepcopy(old)

    while true
        for y = 1:length(old)
            for x = 1:length(old[y])
                occupied = get_adjacent_occupied((y, x), old)

                if old[y][x] == 'L' && isempty(occupied)
                    new[y][x] = '#'
                elseif old[y][x] == '#' && length(occupied) > 3
                    new[y][x] = 'L'
                end
            end
        end

        if old == new
            return old
        end

        old = deepcopy(new)
    end
end

function main(args)
    input = map(line -> collect(line), readlines(args[1]))

    result = count(seat -> seat == '#', collect(Iterators.flatten(simulate(input))))
    @show result
end

main(ARGS)
