fn main() {
    let data = include_str!("../input.txt");
    let answer = part_a(&data);
    println!("part_a: {}", answer);
}

fn part_a(data: &str) -> u32 {
    let lines: Vec<&str> = data.lines().collect();
    let dirs = lines[0].chars().collect::<Vec<char>>();

    let mut network = std::collections::HashMap::new();

    for line in lines[2..].iter() {
        let node_id: &str = &line[0..3];
        let left: &str = &line[7..10];
        let right: &str = &line[12..15];
        network.insert(node_id, (left, right));
    }

    // Recursively find the number of steps to ZZZ
    find_zzz("AAA", &network, &dirs, 0)
}

fn find_zzz(
    node: &str,
    network: &std::collections::HashMap<&str, (&str, &str)>,
    dirs: &Vec<char>,
    step: u32,
) -> u32 {
    let dir = dirs[step as usize % dirs.len()];
    let next_node = match dir {
        'L' => network.get(node).unwrap().0,
        'R' => network.get(node).unwrap().1,
        _ => panic!("bad direction"),
    };
    if next_node == "ZZZ" {
        return step + 1;
    }
    find_zzz(next_node, &network, &dirs, step + 1)
}

#[cfg(test)]
#[test]
fn test_part_a() {
    let data = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(part_a(data), 6);
}
