use rand::prelude::*;
#[derive(Debug,PartialEq)]
pub enum Suit {
    Heart, 
    Diamond,
    Spade ,
    Club
}
#[derive(Debug,PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen ,
    Jack ,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(1..=4);
        Suit::translate(n)
    }

    pub fn translate(value: u8) -> Suit {
        let x = match value {
            1=>Suit::Heart,
            2=>Suit::Diamond,
            3=>Suit::Spade,
        _=>Suit::Club , 
        };
        x
    }
}

impl Rank {

    pub fn random() -> Rank {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(1..=13);
        Rank::translate(n)
    }

    pub fn translate(value: u8) -> Rank {
        let x = match value {
            1 => Rank::Ace ,
            11 => Rank::King ,
            12 => Rank::Queen,
            13 => Rank::Jack ,
            _=> Rank::Number(value),  
        };
        x

    }
}
#[derive(Debug,PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
if card.suit == Suit::Spade && card.rank == Rank::Ace {
    return true
}
false
}