use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    hand_type: HandType,
    cards: [u8; 5],
    bid: u32,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut data = value.split(' ');
        let hand = data.next().unwrap();

        let mut card_counts = [0u8; 15];

        let mut cards = [0u8; 5];
        for (i, card) in hand.chars().enumerate() {
            let card = match card {
                x if  x.is_ascii_digit() => card.to_digit(10).unwrap() as u8,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Invalid card!"),
            };
            cards[i] = card;
            card_counts[card as usize] += 1;
        }

        let mut hand_type = HandType::HighCard;
        for i in 2..15 {
            let count = card_counts[i];
            if count == 5 {
                hand_type = HandType::FiveOfAKind;
                break;
            } else if count == 4 {
                hand_type = HandType::FourOfAKind;
                break;
            } else if count == 3 {
                if hand_type == HandType::OnePair {
                    hand_type = HandType::FullHouse;
                    break;
                }
                hand_type = HandType::ThreeOfAKind;
            } else if count == 2 {
                if hand_type == HandType::ThreeOfAKind {
                    hand_type = HandType::FullHouse;
                    break;
                } else if hand_type == HandType::OnePair {
                    hand_type = HandType::TwoPair;
                } else {
                    hand_type = HandType::OnePair
                }
            }
        }

        let bid = data.next().unwrap().parse().unwrap();

        Self {
            hand_type,
            cards,
            bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_type_ord = self.hand_type.cmp(&other.hand_type);
        if hand_type_ord.is_ne() {
            return hand_type_ord;
        }

        for i in 0..5 {
            let card_ord = self.cards[i].cmp(&other.cards[i]);
            if card_ord.is_ne() {
                return card_ord.reverse();
            }
        }

        return std::cmp::Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let data = fs::read_to_string("res/day_7.txt").unwrap();
    let lines = data.split('\n');

    let mut hands: Vec<Hand> = lines.map(Hand::from).collect();
    hands.sort();
    hands.reverse();

    let mut sum = 0u32;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i + 1) as u32 * hand.bid;
    }

    println!("Part 1: {:?}", sum);
}

