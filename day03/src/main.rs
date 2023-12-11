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

fn part_a(filename: &str) {
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut symbols = Vec::new();
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

            // if we get here, we have a symbol
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
    let sum = find_valid_parts(&parts, &symbols);
    println!("Part a sum: {}", sum);
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

fn main() {
    //let data_file = "example1.txt";
    let data_file = "input.txt";
    part_a(data_file);
}
