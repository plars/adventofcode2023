fn part_a(data: &str) {
    let mut sum = 0;
    for line in data.split_whitespace() {
        let calibration_value = get_calibration_value(line);
        println!("{}", calibration_value);
        sum += calibration_value;
    }
    println!("sum: {}", sum)
}
fn get_calibration_value(line: &str) -> u32 {
    let mut first = 0;
    let mut last = 0;
    let mut v = Vec::new();
    for c in line.chars().filter(|c| c.is_digit(10)) {
        v.push(c.to_digit(10).unwrap());
    }
    if v.len() > 0 {
        first = v[0];
        last = v[v.len() - 1];
    }

    return first * 10 + last;
}

fn part_b(data: &str) {
    let number_words = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];
    let mut sum = 0;
    for line in data.split_whitespace() {
        let mut first = 0;
        let mut last = 0;

        // find the first number
        for i in 0..line.len() {
            for (index, word) in number_words.iter().enumerate() {
                if i + word.len() > line.len() {
                    continue;
                };
                if line[i..i + word.len()].starts_with(word) {
                    first = index / 2 + 1;
                    break;
                };
            }
            if first > 0 {
                break;
            }
        }
        // find the last number
        for i in (0..line.len()).rev() {
            for (index, word) in number_words.iter().enumerate() {
                if i + word.len() > line.len() {
                    continue;
                };
                if line[i..i + word.len()].starts_with(word) {
                    last = index / 2 + 1;
                    break;
                };
            }
            if last > 0 {
                break;
            }
        }
        let calibration_value = first * 10 + last;
        sum += calibration_value;
    }
    println!("sum: {}", sum)
}

fn main() {
    let data = include_str!("../input.txt");
    part_a(data);
    part_b(data);
}
