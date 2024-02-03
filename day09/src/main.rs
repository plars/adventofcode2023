fn main() {
    let data = include_str!("../input.txt");
    let answer = part_a(data).unwrap();
    println!("Part a answer: {answer}");
}

fn part_a(data: &str) -> Option<i32> {
    if data.is_empty() {
        None
    } else {
        let mut result = 0;
        for line in data.lines() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let next_value = find_next_value(&numbers);
            result += next_value;
        }
        Some(result)
    }
}

fn find_next_value(numbers: &Vec<i32>) -> i32 {
    let mut diff_values: Vec<i32> = Vec::new();
    for i in 0..numbers.len() - 1 {
        let difference: i32 = numbers[i + 1] - numbers[i];
        diff_values.push(difference);
    }
    if diff_values.iter().all(|&x| x == 0) {
        return *numbers.last().unwrap();
    }
    // otherwise, we need to recurse to find what we need to add to the last number in our numbers vec
    let next_difference: i32 = find_next_value(&diff_values);
    return next_difference + numbers.last().unwrap();
}

#[cfg(test)]
#[test]
fn test_part_a() {
    let data = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let answer = part_a(data).unwrap();
    assert_eq!(answer, 114);
    println!("Part a answer: {answer}");
}
