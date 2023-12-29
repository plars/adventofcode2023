#[derive(Debug)]
struct Hand {
    cards: String,
    bet: u32,
    card_arr: [u32; 13],
    score: u32,
}

static CARD_TYPES: &str = "23456789TJQKA";

fn main() {
    let data = include_str!("../input.txt");
    let answer = part_a(&data);
    println!("part_a: {}", answer);
}

fn part_a(data: &str) -> u32 {
    let mut hand_vec: Vec<Hand> = Vec::new();
    for line in data.lines() {
        let (hand, bet) = line.split_once(" ").unwrap();
        let mut hand_arr: [u32; 13] = [0; 13];
        for card in hand.chars() {
            hand_arr[CARD_TYPES.find(card).unwrap()] += 1;
        }
        let mut this_hand = Hand {
            cards: hand.to_string(),
            bet: bet.parse::<u32>().unwrap(),
            card_arr: hand_arr,
            score: 0,
        };
        score_hand(&mut this_hand);
        hand_vec.push(this_hand);
    }
    hand_vec.sort_by(|a, b| a.score.cmp(&b.score));
    get_total(&hand_vec)
}

fn score_hand(hand: &mut Hand) {
    // First, let's get the big score based on the hand type
    for (i, x) in hand.card_arr.iter().enumerate() {
        match x {
            5 => hand.score = 100000000 + i as u32,
            4 => hand.score += 50000000 + i as u32,
            3 => hand.score += 20000000 + i as u32,
            2 => hand.score += 5000000 + i as u32,
            _ => (),
        }
    }
    // Weird rule, but the tie-breaker is based on the high card in priority of position in the hand
    // So let's modify the score based on that a bit, but not so much that it would override the hand type score
    for (i, card) in hand.cards.chars().enumerate() {
        hand.score += 13_u32.pow(4 - i as u32) * (CARD_TYPES.find(card).unwrap() as u32 + 1);
    }
}

fn get_total(hand_vec: &Vec<Hand>) -> u32 {
    let mut total: u32 = 0;
    for (i, hand) in hand_vec.iter().enumerate() {
        total += hand.bet * (1 + i as u32);
    }
    total
}

#[cfg(test)]
#[test]
fn test_part_a() {
    let data = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let answer = part_a(data);
    assert_eq!(answer, 6440);
}
