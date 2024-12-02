use std::cmp::Ordering;
use std::{error::Error, fmt};

pub const POKER_HAND_SIZE: usize = 5;

#[derive(Debug)]
pub struct PokerError(pub String);
impl Error for PokerError {}
impl fmt::Display for PokerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "oh boy, your poker is miserable: {}", self.0)
    }
}

pub trait CardSorting: PartialEq + PartialOrd + Eq + Ord + fmt::Debug + Clone + TryFrom<char> {}
impl<T> CardSorting for T where T: PartialEq + PartialOrd + Eq + Ord + fmt::Debug + Clone + TryFrom<char> {}
pub trait CardTrait<T>: CardSorting { fn get_hand_type(cards: &mut [T]) -> Result<HandType, PokerError>; }


#[derive(PartialEq, PartialOrd, Eq, Debug, Ord)]
pub enum HandType { HighCard, OnePair, TwoPair, ThreeofAKind, FullHouse, FourOfAKind, FiveOfAKind }

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand<T: CardTrait<T>> {
    cards: Vec<T>,
    typ: HandType,
    pub bid: u64,
}
impl <T: CardTrait<T>> TryFrom<&HandInput> for Hand<T> {
    type Error = PokerError;

    fn try_from(given: &HandInput) -> Result<Self, Self::Error> {
        let chrs = given.card_string.chars(); let n = chrs.clone().count(); 
        if n != POKER_HAND_SIZE { return Err(PokerError(format!("must give five chars for Pokerhand: {n}"))) }

        let mut cards: Vec<T> = Vec::with_capacity(POKER_HAND_SIZE);
        given.card_string.chars().map(|c| T::try_from(c)).try_for_each(|card| -> Result<(), PokerError> {
            let a = card.map_err(|_| PokerError("nooo".to_string()))?; // trait wonkyness https://stackoverflow.com/q/59214210
            cards.push(a);
            Ok(())
        })?;
        let typ = T::get_hand_type(&mut cards)?;
        let bid = given.bid;
        Ok(Self{cards, typ, bid})
    }
}
impl <T: CardTrait<T>> Hand<T> {
    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_ordering = self.typ.cmp(&other.typ);
        if hand_ordering.is_eq() {
            for idx in 0..POKER_HAND_SIZE {
                let o = self.cards[idx].cmp(&other.cards[idx]);
                if o != Ordering::Equal { return o; }
            }
            Ordering::Equal // should not happen actually
        } else {
            hand_ordering
        }
    }
}

pub struct HandInput {
    card_string: String,
    bid: u64
}
impl HandInput {
    pub const fn new(card_string: String, bid: u64) -> Self {
        Self{card_string, bid}
    }
}
