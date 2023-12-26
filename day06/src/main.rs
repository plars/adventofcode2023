fn main() {
    let real_time_arr: [u64; 4] = [49, 78, 79, 80];
    let real_distance_arr: [u64; 4] = [298, 1185, 1066, 1181];
    let answer = part_a(&real_time_arr, &real_distance_arr);
    println!("Part a answer: {answer}");
    let real_time_arr: [u64; 1] = [49787980];
    let real_distance_arr: [u64; 1] = [298118510661181];
    let answer = part_a(&real_time_arr, &real_distance_arr);
    println!("Part b answer: {answer}");
}

fn part_a(time_arr: &[u64], distance_arr: &[u64]) -> u64 {
    let mut wins = Vec::new();
    wins.resize(time_arr.len(), 0);

    for race_num in 0..time_arr.len() {
        for hold_time in 1..time_arr[race_num] {
            if hold_time * (time_arr[race_num] - hold_time) > distance_arr[race_num] {
                wins[race_num] += 1;
            }
        }
    }
    // return the product of elements in wins
    wins.iter().product()
}

#[cfg(test)]
#[test]
fn test_part_a() {
    let time_arr = [7, 15, 30];
    let distance_arr = [9, 40, 200];
    assert_eq!(part_a(&time_arr, &distance_arr), 288);
}

#[test]
fn test_part_b() {
    let time_arr = [71530];
    let distance_arr = [940200];
    assert_eq!(part_a(&time_arr, &distance_arr), 71503);
}
