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

fn main() {
    let data = include_str!("../input.txt");
    part_a(data);
}
