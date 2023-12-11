use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandType {
    fn val(&self) -> i32 {
        match self {
            HandType::FiveKind => 0,
            HandType::FourKind => 1,
            HandType::FullHouse => 2,
            HandType::ThreeKind => 3,
            HandType::TwoPair => 4,
            HandType::OnePair => 5,
            HandType::HighCard => 6,
        }
    }
}

fn hand_type(hand: [i32; 5]) -> HandType {
    let mut sorted = hand.clone();
    let has_joker = hand.contains(&1);
    // Sort by number of occurrences, then by rank
    sorted.sort_by_key(|&c| -(hand.iter().filter(|&&d| c == d).count() as i32 * 15 + c));
    if has_joker {
        // Always best to replace with the most common non-joker card
        let first_nonjoker = sorted.iter().find(|&&card| card != 1);
        if let Some(&to_replace) = first_nonjoker {
            sorted = sorted.map(|card| {
                if card == 1 {
                    to_replace
                } else {
                    card
                }
            });
        }
        // If all cards are jokers, it doesn't matter what kind of 5k it is.
        sorted.sort_by_key(|&c| -(hand.iter().filter(|&&d| c == d).count() as i32 * 15 + c));
    }
    if sorted[4] == sorted[0] {
        HandType::FiveKind
    } else if sorted[3] == sorted[0] {
        HandType::FourKind
    } else if sorted[2] == sorted[0] && sorted[3] == sorted[4] {
        HandType::FullHouse
    } else if sorted[2] == sorted[0] {
        HandType::ThreeKind
    } else if sorted[1] == sorted[0] && sorted[3] == sorted[2] {
        HandType::TwoPair
    } else if sorted[1] == sorted[0] {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

pub fn pt1(input: String) {
    main(input, false);
}

pub fn pt2(input: String) {
    main(input, true);
}

fn main(input: String, j_wild: bool) {
    let mut hands: Vec<(Vec<i32>, i32)> = input
        .split("\r\n")
        .map(|hand| hand.split(" ").collect::<Vec<&str>>())
        .map(|hand| {
            (
                hand[0]
                    .chars()
                    .map(|c| match c {
                        'K' => 13,
                        'Q' => 12,
                        'J' => if j_wild {1} else {11},
                        'T' => 10,
                        'A' => 14, //Aces high
                        other => other.to_string().parse::<i32>().unwrap(),
                    })
                    .collect::<Vec<i32>>(),
                hand[1].parse::<i32>().unwrap(),
            )
        })
        .collect();
    let hand_types: HashMap<Vec<i32>, HandType> = HashMap::from_iter(
        hands
            .iter()
            .map(|(hand, _)| (hand.clone(), hand_type(hand[..].try_into().unwrap()))),
    );
    hands.sort_by_key(|(hand, _)| {
        hand.iter()
            .fold(-hand_types.get(hand).unwrap().val(), |acc, &c| acc * 15 + c)
    });
    let win_total = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, (_, bid))| acc + ((idx as i32 + 1) * bid));

    println!("Hand values: {win_total}");
}
