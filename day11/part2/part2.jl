STEPS = [(0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)]

function find_first_seat(seat, dir, map)
    (y, x) = seat
    (dy, dx) = dir
    (iy, ix) = (y, x)
    iter = 1

    while true
        iy = y + (iter * dy)
        ix = x + (iter * dx)

        if 1 <= iy <= length(map) && 1 <= ix <= length(map[y])
            if map[iy][ix] == '#' || map[iy][ix] == 'L'
                return map[iy][ix]
            end
        else
            return '.'
        end
        
        iter += 1
    end
end

function get_adjacent_occupied(seat, map)
    return filter(adjacent -> adjacent == '#', [find_first_seat(seat, dir, map) for dir in STEPS])
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
                elseif old[y][x] == '#' && length(occupied) > 4
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
