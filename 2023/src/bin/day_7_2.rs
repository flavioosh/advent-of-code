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
                'J' => 1,
                x if x.is_ascii_digit() => card.to_digit(10).unwrap() as u8,
                'T' => 10,
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                _ => panic!("Invalid card!"),
            };
            cards[i] = card;
            card_counts[card as usize] += 1;
        }

        let jokers = card_counts[1];
        let hand_type = match jokers {
            5 | 4 => HandType::FiveOfAKind,
            3 => {
                let mut best_hand_type = HandType::FourOfAKind;
                for &count in card_counts.iter().take(14).skip(2) {
                    if count == 2 {
                        best_hand_type = HandType::FiveOfAKind;
                        break;
                    }
                }
                best_hand_type
            }
            2 => {
                let mut best_hand_type = HandType::ThreeOfAKind;
                for &count in card_counts.iter().take(14).skip(2) {
                    if count == 3 {
                        best_hand_type = HandType::FiveOfAKind;
                        break;
                    } else if count == 2 {
                        best_hand_type = HandType::FourOfAKind;
                        break;
                    } else if count == 1 {
                        best_hand_type = HandType::ThreeOfAKind;
                    }
                }
                best_hand_type
            }
            1 => {
                let mut best_hand_type = HandType::OnePair;
                for &count in card_counts.iter().take(14).skip(2) {
                    if count == 4 {
                        best_hand_type = HandType::FiveOfAKind;
                        break;
                    } else if count == 3 {
                        best_hand_type = HandType::FourOfAKind;
                        break;
                    } else if count == 2 {
                        if best_hand_type == HandType::ThreeOfAKind {
                            best_hand_type = HandType::FullHouse;
                            break;
                        }
                        best_hand_type = HandType::ThreeOfAKind;
                    }
                }
                best_hand_type
            }
            _ => {
                let mut best_hand_type = HandType::HighCard;
                for &count in card_counts.iter().take(14).skip(2) {
                    if count == 5 {
                        best_hand_type = HandType::FiveOfAKind;
                        break;
                    } else if count == 4 {
                        best_hand_type = HandType::FourOfAKind;
                        break;
                    } else if count == 3 {
                        if best_hand_type == HandType::OnePair {
                            best_hand_type = HandType::FullHouse;
                            break;
                        }
                        best_hand_type = HandType::ThreeOfAKind;
                    } else if count == 2 {
                        if best_hand_type == HandType::ThreeOfAKind {
                            best_hand_type = HandType::FullHouse;
                            break;
                        } else if best_hand_type == HandType::OnePair {
                            best_hand_type = HandType::TwoPair;
                        } else {
                            best_hand_type = HandType::OnePair
                        }
                    }
                }
                best_hand_type
            }
        };

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

        std::cmp::Ordering::Equal
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

    println!("Part 2: {}", sum);
}
