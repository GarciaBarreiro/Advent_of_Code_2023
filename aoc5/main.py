import re

with open('./input', newline='') as input:
    lines = input.readlines()
    line = lines[0]
    reg = re.split('\s+', line)
    seeds = []
    for r in reg:
        if r != reg[0] and r:
            seeds.append(int(r))
    map = 0
    map_1 = []
    map_2 = []
    map_3 = []
    map_4 = []
    map_5 = []
    map_6 = []
    map_7 = []
    for line in lines:
        if "seed-to-soil map" in line:
            map = 1
        elif "soil-to-fertilizer map" in line:
            map = 2
        elif "fertilizer-to-water map" in line:
            map = 3
        elif "water-to-light map" in line:
            map = 4
        elif "light-to-temperature map" in line:
            map = 5
        elif "temperature-to-humidity map" in line:
            map = 6
        elif "humidity-to-location map" in line:
            map = 7
        reg = re.split('\s+', line)

        if len(reg) == 4:
            match map:
                case 1:
                    map_1.append([
                            int(reg[0]),
                            int(reg[1]),
                            int(reg[2])
                            ])
                case 2:
                    map_2.append([
                            int(reg[0]),
                            int(reg[1]),
                            int(reg[2])
                            ])
                case 3:
                    map_3.append([
                            int(reg[0]),
                            int(reg[1]),
                            int(reg[2])
                            ])
                case 4:
                    map_4.append([
                            int(reg[0]),
                            int(reg[1]),
                            int(reg[2])
                            ])
                case 5:
                    map_5.append([
                            int(reg[0]),
                            int(reg[1]),
                            int(reg[2])
                            ])
                case 6:
                    map_6.append([
                            int(reg[0]),
                            int(reg[1]),
                            int(reg[2])
                            ])
                case 7:
                    map_7.append([
                            int(reg[0]),
                            int(reg[1]),
                            int(reg[2])
                            ])
                case _:
                    continue
    lowest_location = 99999999999999999999999999
    for seed in seeds:
        for map in map_1:
            if seed >= map[1] and seed < map[1]+map[2]:
                seed = seed - map[1] + map[0]
                break
        for map in map_2:
            if seed >= map[1] and seed < map[1]+map[2]:
                seed = seed - map[1] + map[0]
                break
        for map in map_3:
            if seed >= map[1] and seed < map[1]+map[2]:
                seed = seed - map[1] + map[0]
                break
        for map in map_4:
            if seed >= map[1] and seed < map[1]+map[2]:
                seed = seed - map[1] + map[0]
                break
        for map in map_5:
            if seed >= map[1] and seed < map[1]+map[2]:
                seed = seed - map[1] + map[0]
                break
        for map in map_6:
            if seed >= map[1] and seed < map[1]+map[2]:
                seed = seed - map[1] + map[0]
                break
        for map in map_7:
            if seed >= map[1] and seed < map[1]+map[2]:
                seed = seed - map[1] + map[0]
                break
        if seed < lowest_location: lowest_location = seed
    print("one star sol:", lowest_location)

# this takes a lot of time, consider optimizing it
# maybe go backwards? from the lowest possible location (0) until one valid is found
with open('./input', newline='') as input:
    lines = input.readlines()
    line = lines[0]
    reg = re.split('\s+', line)
    seed_ranges = []
    for r in reg:
        if r != reg[0] and r:
            seed_ranges.append(int(r))
    seeds = []
    i = 0
    for i in range(len(seed_ranges)):
        if i % 2 == 0: seeds.append([seed_ranges[i], seed_ranges[i+1]])
        i += 2

    map = 0
    map_1 = []
    map_2 = []
    map_3 = []
    map_4 = []
    map_5 = []
    map_6 = []
    map_7 = []
    for line in lines:
        if "seed-to-soil map" in line:
            map = 1
        elif "soil-to-fertilizer map" in line:
            map = 2
        elif "fertilizer-to-water map" in line:
            map = 3
        elif "water-to-light map" in line:
            map = 4
        elif "light-to-temperature map" in line:
            map = 5
        elif "temperature-to-humidity map" in line:
            map = 6
        elif "humidity-to-location map" in line:
            map = 7
        reg = re.split('\s+', line)

        if len(reg) == 4:
            match map:
                case 1:
                    map_1.append([
                            int(reg[0]),
                            int(reg[1]),
                            int(reg[2])
                            ])
                case 2:
                    map_2.append([
                            int(reg[0]),
                            int(reg[1]),
                            int(reg[2])
                            ])
                case 3:
                    map_3.append([
                            int(reg[0]),
                            int(reg[1]),
                            int(reg[2])
                            ])
                case 4:
                    map_4.append([
                            int(reg[0]),
                            int(reg[1]),
                            int(reg[2])
                            ])
                case 5:
                    map_5.append([
                            int(reg[0]),
                            int(reg[1]),
                            int(reg[2])
                            ])
                case 6:
                    map_6.append([
                            int(reg[0]),
                            int(reg[1]),
                            int(reg[2])
                            ])
                case 7:
                    map_7.append([
                            int(reg[0]),
                            int(reg[1]),
                            int(reg[2])
                            ])
                case _:
                    continue
    lowest_location = 99999999999999999999999999
    for s_range in seeds:
        seed = s_range[0]
        i = 0
        while seed < s_range[0]+s_range[1]:
            for map in map_1:
                if seed >= map[1] and seed < map[1]+map[2]:
                    seed = seed - map[1] + map[0]
                    break
            for map in map_2:
                if seed >= map[1] and seed < map[1]+map[2]:
                    seed = seed - map[1] + map[0]
                    break
            for map in map_3:
                if seed >= map[1] and seed < map[1]+map[2]:
                    seed = seed - map[1] + map[0]
                    break
            for map in map_4:
                if seed >= map[1] and seed < map[1]+map[2]:
                    seed = seed - map[1] + map[0]
                    break
            for map in map_5:
                if seed >= map[1] and seed < map[1]+map[2]:
                    seed = seed - map[1] + map[0]
                    break
            for map in map_6:
                if seed >= map[1] and seed < map[1]+map[2]:
                    seed = seed - map[1] + map[0]
                    break
            for map in map_7:
                if seed >= map[1] and seed < map[1]+map[2]:
                    seed = seed - map[1] + map[0]
                    break
            if seed < lowest_location: lowest_location = seed
            i += 1
            seed = s_range[0] + i
    print("two star sol:", lowest_location)
