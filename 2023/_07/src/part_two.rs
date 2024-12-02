use std::fs::read_to_string;
use crate::poker::{self, CardTrait, Hand, HandInput, HandType, PokerError, POKER_HAND_SIZE};
use itertools::Itertools;

pub fn run(input_file: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let body = read_to_string(input_file)?;
    let mut game = parse(&body)?;
    Ok(solve(&mut game))
}

fn parse(body: &str) -> Result<Vec<Hand<CardType>>, Box<dyn std::error::Error>> {
    let mut game: Vec<Hand<CardType>> = Vec::new();
    for line in body.lines() {
        let (cards, bid) = line.split_at(5);
        
        let input = HandInput::new(String::from(cards), bid.trim().parse()?);
        game.push(Hand::try_from(&input)?);
    }
    Ok(game)
}

fn solve(game: &mut [Hand<CardType>]) -> u64 {
    game.sort_by(poker::Hand::cmp);
    game.iter().enumerate().map(|(i, hand)| (i as u64 + 1) * hand.bid).sum()
}

// =================================================================================================

#[derive(PartialEq, PartialOrd, Eq, Debug, Clone, Copy, Ord)]
enum CardType { J, Two, Three, Four, Five, Six, Seven, Eight, Nine, T, Q, K, A }

// awesome try from trait: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
// https://dev.to/peterblockman/quick-guide-to-rusts-frominto-and-tryfromtryinto-traits-3gf1
impl TryFrom<char> for CardType { 
    type Error = PokerError;

    fn try_from(c: char) -> Result<Self, PokerError> {
        Ok(match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => { return Err(PokerError(format!("invalid card type {c}"))); }
        })
    }

}

impl CardTrait<Self> for CardType {
    fn get_hand_type(cards: &mut [Self]) -> Result<HandType, PokerError> {
        if cards.len() != POKER_HAND_SIZE { return Err(PokerError(format!("you're not holding five cards, brah: {}", cards.len()))); }
        cards.sort(); // must sort for later grouping

        let mut keys_lengths: Vec<(Self, usize)> = Vec::new();
        for (key, group) in &cards.iter().group_by(|t| *t) {
            keys_lengths.push((*key, group.count()));
        }
        keys_lengths.sort_by_key(|kl| kl.1);

        // accounts for JOKERs
        if keys_lengths.iter().any(|x| x.0 == Self::J) && keys_lengths.len() > 1 {
            let jidx = keys_lengths.iter().position(|&k| k.0 == Self::J)
                .ok_or_else(|| PokerError("there are supposed to be JOKERs in here".to_string()))?;

            let n = keys_lengths.len();
            if jidx == (n - 1) {
                keys_lengths[jidx-1].1 += keys_lengths[jidx].1; // add J count to the group below
            } else {
                keys_lengths[n-1].1 += keys_lengths[jidx].1; // add J count to the group above
            }
            keys_lengths.remove(jidx);
        }

        let hand_type = match keys_lengths.len() {
            5 => Ok(HandType::HighCard),
            4 => Ok(HandType::OnePair),
            3 => { if keys_lengths[2].1 == 3 { Ok(HandType::ThreeofAKind) } else { Ok(HandType::TwoPair) } }
            2 => { if keys_lengths[1].1 == 4 { Ok(HandType::FourOfAKind) } else { Ok(HandType::FullHouse) } }
            1 => Ok(HandType::FiveOfAKind),
            _ => Err(PokerError(format!("grouping of five cards must yield between 1 to 5 groups, got {}", keys_lengths.len()))),
        }?;
        Ok(hand_type)
    }
}