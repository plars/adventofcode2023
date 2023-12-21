#[derive(Debug)]
struct AlmanacStruct {
    seeds: Vec<u32>,
    seed_soil_map: Vec<CategoryMap>,
    soil_fertilizer_map: Vec<CategoryMap>,
    fertilizer_water_map: Vec<CategoryMap>,
    water_light_map: Vec<CategoryMap>,
    light_temperature_map: Vec<CategoryMap>,
    temperature_humidity_map: Vec<CategoryMap>,
    humidity_location_map: Vec<CategoryMap>,
}

#[derive(Debug)]
struct CategoryMap {
    src_range: std::ops::Range<i64>,
    offset: i64,
}

fn main() {
    let data = include_str!("../input.txt");
    let answer = part_a(data);
    println!("Part a answer: {answer}");
}

fn part_a(data: &str) -> i64 {
    let lines = data.lines().collect();
    let almanac_data: AlmanacStruct = parse_all(&lines);
    // The result is the minimum of all of the mappings applied to all the seeds
    let result = &almanac_data
        .seeds
        .iter()
        .map(|s| get_seed_location(&almanac_data, *s))
        .min()
        .unwrap();
    return *result;
}

fn parse_all(lines: &Vec<&str>) -> AlmanacStruct {
    let seeds = lines[0]
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut all_maps: Vec<Vec<CategoryMap>> = Vec::new();
    let mut index = 3; //map data starts on line 3
    while index < lines.len() - 1 {
        let mut this_map: Vec<CategoryMap> = Vec::new();
        while !lines[index].is_empty() {
            let line_data: Vec<i64> = lines[index]
                .split(" ")
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            let dest = line_data[0];
            let src = line_data[1];
            let length = line_data[2];
            dbg!(dest, src);
            this_map.push(CategoryMap {
                src_range: src..src + length,
                offset: dest - src, // because we did it this way, we need to ADD the offset later
            });
            index += 1;
            if index >= lines.len() - 1 {
                break;
            }
        }
        all_maps.push(this_map);
        index += 2;
    }
    let humidity_location_map = all_maps.pop().unwrap();
    let temperature_humidity_map = all_maps.pop().unwrap();
    let light_temperature_map = all_maps.pop().unwrap();
    let water_light_map = all_maps.pop().unwrap();
    let fertilizer_water_map = all_maps.pop().unwrap();
    let soil_fertilizer_map = all_maps.pop().unwrap();
    let seed_soil_map = all_maps.pop().unwrap();

    AlmanacStruct {
        seeds,
        seed_soil_map,
        soil_fertilizer_map,
        fertilizer_water_map,
        water_light_map,
        light_temperature_map,
        temperature_humidity_map,
        humidity_location_map,
    }
}

fn get_seed_location(almanac_data: &AlmanacStruct, seed: u32) -> i64 {
    let value = apply_maps(seed as i64, &almanac_data.seed_soil_map);
    let value = apply_maps(value, &almanac_data.soil_fertilizer_map);
    let value = apply_maps(value, &almanac_data.fertilizer_water_map);
    let value = apply_maps(value, &almanac_data.water_light_map);
    let value = apply_maps(value, &almanac_data.light_temperature_map);
    let value = apply_maps(value, &almanac_data.temperature_humidity_map);
    let value = apply_maps(value, &almanac_data.humidity_location_map);
    return value;
}

fn apply_maps(value: i64, maps: &Vec<CategoryMap>) -> i64 {
    for map in maps {
        if map.src_range.contains(&value) {
            return value + map.offset;
        }
    }
    // otherwise, this means it isn't in a map range, so it's the original value
    return value;
}

#[cfg(test)]
#[test]
fn test_part_a() {
    let data = include_str!("../example1.txt");
    let answer = part_a(data);
    println!("Part a answer: {answer}");
    assert_eq!(answer, 35);
}
