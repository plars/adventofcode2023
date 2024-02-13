use std::io::BufRead;

#[derive(Debug)]
struct Symbol {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Part {
    x: i32,
    y: i32,
    value: i32,
    length: usize,
}

fn main() {
    let data_file = "input.txt";
    let (sum_a, sum_b) = part_a_b(data_file);
    println!("Part a sum: {}", sum_a);
    println!("Part b sum: {}", sum_b);
}

fn part_a_b(filename: &str) -> (i32, i32) {
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut symbols = Vec::new();
    let mut gears = Vec::new();
    let mut parts = Vec::new();
    let mut y: i32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        // we'll be using a pre-increment, so start at -1
        let mut x: i32 = -1;
        let mut current_num: i32 = 0;
        for c in line.chars() {
            x += 1;
            if c.is_ascii_digit() {
                current_num = current_num * 10 + c.to_digit(10).unwrap() as i32;
                continue;
            }

            // check if we just completed a number, if so, add it to the parts vector
            if current_num > 0 {
                let length = current_num.to_string().len();
                // make sure we're at the beginning of the number
                let start_x = x - length as i32;
                parts.push(Part {
                    x: start_x,
                    y,
                    value: current_num,
                    length,
                });
                current_num = 0;
            }

            // normal case, nothing interesting
            if c == '.' {
                continue;
            }

            // save the gears in a separate vector
            if c == '*' {
                gears.push(Symbol { x, y });
            }

            // push all symbols here, including gears
            symbols.push(Symbol { x, y });
        }
        // also handle the case where the line ends with a number
        if current_num > 0 {
            let length = current_num.to_string().len();
            let start_x = x - length as i32;
            parts.push(Part {
                x: start_x,
                y,
                value: current_num,
                length,
            });
        }
        y += 1;
    }
    let sum_a = find_valid_parts(&parts, &symbols);
    let sum_b = find_gear_ratio_sum(&parts, &gears);
    (sum_a, sum_b)
}

fn find_valid_parts(parts: &Vec<Part>, symbols: &Vec<Symbol>) -> i32 {
    let mut sum = 0;
    for part in parts {
        for symbol in symbols {
            if symbol.x >= part.x - 1
                && symbol.x <= part.x + part.length as i32
                && symbol.y >= part.y - 1
                && symbol.y <= part.y + 1
            {
                sum += part.value;
                break;
            }
        }
    }
    sum
}

fn find_gear_ratio_sum(parts: &Vec<Part>, gears: &Vec<Symbol>) -> i32 {
    let mut sum = 0;
    for gear in gears {
        let mut gear_ratio = 0;
        let mut num_gears = 0;
        let mut sum_for_this_gear = 0;
        for part in parts {
            if gear.x >= part.x - 1
                && gear.x <= part.x + part.length as i32
                && gear.y >= part.y - 1
                && gear.y <= part.y + 1
            {
                num_gears += 1;
                if num_gears > 2 {
                    // this isn't valid, because it's next to more than to parts
                    sum_for_this_gear = 0;
                    break;
                }
                if num_gears == 1 {
                    // first gear, just save the value
                    gear_ratio = part.value;
                    continue;
                }
                gear_ratio *= part.value;
                sum_for_this_gear += gear_ratio;
            }
        }
        sum += sum_for_this_gear;
    }
    sum
}

#[cfg(test)]
#[test]
fn test_part_a_b() {
    let (sum_a, sum_b) = part_a_b("example1.txt");
    assert_eq!(sum_a, 4361);
    assert_eq!(sum_b, 467835);
}
