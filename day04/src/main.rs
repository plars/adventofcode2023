fn main() {
    //let data = include_str!("../example1.txt");
    let data = include_str!("../input.txt");
    part_a(data);
}

fn part_a(data: &str) {
    let mut sum = 0;
    for line in data.lines() {
        let mut winning_numbers = Vec::new();
        let mut card_numbers = Vec::new();
        // split on ": " and just get the second part
        let parts: Vec<&str> = line.split(": ").nth(1).unwrap().split(" | ").collect();
        parts[0].split_whitespace().for_each(|n| {
            winning_numbers.push(n.parse::<u32>().unwrap());
        });
        parts[1].split_whitespace().for_each(|n| {
            card_numbers.push(n.parse::<u32>().unwrap());
        });
        let card_value = card_numbers.iter().fold(0, |acc, n| {
            // one winning number gets 1 point, each additional winning number doubles the points
            if winning_numbers.contains(n) {
                match acc {
                    0 => 1,
                    _ => acc * 2,
                }
            } else {
                acc
            }
        });
        sum += card_value;
    }
    println!("sum: {}", sum);
}
