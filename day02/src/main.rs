use std::cmp::max;

fn part_a(data: &str) -> u32 {
    let mut sum = 0;
    let mut game_number = 1;
    for line in data.lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        if line.is_empty() {
            continue;
        }

        // experimenting with longer expressions
        // let's not talk about how long it took me to sort all this out! :)
        let rounds = line
            .split_once(": ")
            .unwrap()
            .1
            .split("; ")
            .flat_map(|round| {
                round
                    .split(", ")
                    .map(|count| {
                        let (num, color) = count.split_once(' ').unwrap();
                        (num.parse::<u32>().unwrap(), color)
                    })
                    .collect::<Vec<_>>()
            });

        // So if I didn't mess that up, we should have an iterator of tuple(u32, &str)
        for (num, color) in rounds {
            match color {
                "red" => red = max(red, num),
                "green" => green = max(green, num),
                "blue" => blue = max(blue, num),
                _ => (),
            }
        }
        if red <= 12 && green <= 13 && blue <= 14 {
            println!(
                "VALID Game {}: red: {}, green: {}, blue: {}",
                game_number, red, green, blue
            );
            sum += game_number;
        }
        game_number += 1;
    }
    sum
}

fn main() {
    //let data = include_str!("../example1.txt");
    let data = include_str!("../input.txt");
    let sum_a = part_a(data);
    println!("Part a sum: {}", sum_a);
}
