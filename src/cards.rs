pub mod deck;
pub mod hand;
pub mod pile;

use std::fmt::{Debug, Formatter};
use std::mem::transmute;
use std::slice::Iter;

/// # Internal representation
///
/// The playing card is internally represented by an unsigned 8-bit integer: \
/// `00SSRRRR` \
/// where `SS` stands for [Suit] (`00` for spades, `01` for hearts, `10` for diamonds, `11` for clubs)
/// and `RRRR` stands for [Rank] (`0001` for ace, up to `1101` for king).
///
/// A special case is for jokers, where `RRRR` is set to `1111` and the most significant bits can be
/// used to differentiate the jokers. For up to three jokers, you can use [JokerColor] to differentiate
/// red joker, black joker, and white joker.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Card(u8);

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_ascii().unwrap_or(self.0.to_string()))
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl TryFrom<u8> for Suit {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (0..=3).contains(&value) {
            Ok(unsafe { transmute(value) })
        } else {
            Err(())
        }
    }
}

impl Suit {
    pub fn iter() -> Iter<'static, Suit> {
        static SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
        SUITS.iter()
    }

    pub fn name(&self) -> &'static str {
        match self {
            Suit::Spades => "spades",
            Suit::Hearts => "hearts",
            Suit::Diamonds => "diamonds",
            Suit::Clubs => "clubs",
        }
    }

    pub fn letter(&self) -> char {
        match self {
            Suit::Spades => 's',
            Suit::Hearts => 'h',
            Suit::Diamonds => 'd',
            Suit::Clubs => 'c',
        }
    }

    pub fn unicode_black(&self) -> char {
        match self {
            Suit::Spades => '♠',
            Suit::Hearts => '♥',
            Suit::Diamonds => '♦',
            Suit::Clubs => '♣',
        }
    }

    pub fn unicode_white(&self) -> char {
        match self {
            Suit::Spades => '♤',
            Suit::Hearts => '♡',
            Suit::Diamonds => '♢',
            Suit::Clubs => '♧',
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rank {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl TryFrom<u8> for Rank {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if ((Rank::Ace as u8)..=(Rank::King as u8)).contains(&value) {
            Ok(unsafe { transmute(value) })
        } else {
            Err(())
        }
    }
}

impl Rank {
    pub fn iter() -> Iter<'static, Rank> {
        static RANKS: [Rank; 13] = [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];
        RANKS.iter()
    }

    pub fn name(&self) -> &'static str {
        match self {
            Rank::Ace => "ace",
            Rank::Two => "two",
            Rank::Three => "three",
            Rank::Four => "four",
            Rank::Five => "five",
            Rank::Six => "six",
            Rank::Seven => "seven",
            Rank::Eight => "eight",
            Rank::Nine => "nine",
            Rank::Ten => "ten",
            Rank::Jack => "jack",
            Rank::Queen => "queen",
            Rank::King => "king",
        }
    }

    pub fn index(&self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum JokerColor {
    Red = 1,
    Black,
    White,
}

impl TryFrom<u8> for JokerColor {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (1..=3).contains(&value) {
            Ok(unsafe { transmute(value) })
        } else {
            Err(())
        }
    }
}

impl JokerColor {
    pub fn name(&self) -> &'static str {
        match self {
            JokerColor::Red => "red",
            JokerColor::Black => "black",
            JokerColor::White => "white",
        }
    }
}

impl Card {
    #[inline]
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card(((suit as u8) << 4) | (rank as u8))
    }

    #[inline]
    pub fn new_joker(color: JokerColor) -> Self {
        Card(((color as u8) << 4) | 0b1111)
    }

    /// Returns whether this card is a standard card (i.e. not joker).
    pub fn is_standard_card(&self) -> bool {
        (self.0 & 0b1111) <= (Rank::King as u8)
    }

    /// Returns whether this card is a joker.
    pub fn is_joker(&self) -> bool {
        (self.0 & 0b1111) == 0b1111
    }

    pub fn suit(&self) -> Option<Suit> {
        if self.is_standard_card() {
            Suit::try_from(self.0 >> 4).ok()
        } else {
            None
        }
    }

    pub fn rank(&self) -> Option<Rank> {
        if self.is_standard_card() {
            Rank::try_from(self.0 & 0b1111).ok()
        } else {
            None
        }
    }

    pub fn joker_color(&self) -> Option<JokerColor> {
        if self.is_joker() {
            JokerColor::try_from(self.0 >> 4).ok()
        } else {
            None
        }
    }

    pub fn to_unicode(&self) -> Result<char, ()> {
        if self.is_standard_card() {
            let suit_val = self.suit().ok_or(())? as u8;
            let mut rank_val = self.rank().ok_or(())? as u8;
            if rank_val > Rank::Jack as u8 {
                rank_val += 1; // Unicode also has rare Knight card suit between jack and queen
            }
            let mut value: u32 = 0x1F0A0;
            value += (suit_val << 4) as u32;
            value += rank_val as u32;
            Ok(char::from_u32(value).ok_or(())?)
        } else {
            let joker_color = self.joker_color().ok_or(())? as u8;
            let mut value: u32 = 0x1F0AF;
            value += (joker_color << 4) as u32;
            Ok(char::from_u32(value).ok_or(())?)
        }
    }

    pub fn to_suit_rank(&self) -> Result<String, ()> {
        if self.is_standard_card() {
            let suit = self.suit().ok_or(())?.unicode_black();
            let rank = self.rank().ok_or(())?.index();
            Ok(format!("{}{}", suit, rank))
        } else {
            let joker_color = self.joker_color().ok_or(())? as u8;
            Ok(format!("🃏{}", joker_color))
        }
    }

    pub fn to_ascii(&self) -> Result<String, ()> {
        if self.is_standard_card() {
            let suit = self.suit().ok_or(())?.letter();
            let rank = self.rank().ok_or(())?.index();
            Ok(format!("{}{}", rank, suit))
        } else {
            let joker_color = self.joker_color().ok_or(())? as u8;
            Ok(format!("J{}", joker_color))
        }
    }

    pub fn name(&self) -> Result<String, ()> {
        if self.is_standard_card() {
            let suit_name = self.suit().ok_or(())?.name();
            let rank_name = self.rank().ok_or(())?.name();
            Ok(format!("{} of {}", rank_name, suit_name))
        } else {
            let color_name = self.joker_color().ok_or(())?.name();
            Ok(format!("{} joker", color_name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let card1 = Card::new(Suit::Diamonds, Rank::King);
        let card2 = Card { 0: 0b10_1101 };
        assert_eq!(card1, card2);
        assert!(card1.is_standard_card());
        assert_eq!(card1.suit(), Some(Suit::Diamonds));
        assert_eq!(card1.rank(), Some(Rank::King));

        let card3 = Card::new_joker(JokerColor::Red);
        assert_ne!(card1, card3);
    }

    #[test]
    fn unicode() {
        let card1 = Card::new(Suit::Diamonds, Rank::King);
        assert_eq!(card1.to_unicode().unwrap(), '🃎');
        assert_eq!(card1.to_suit_rank().unwrap(), "♦K");
        assert_eq!(card1.to_ascii().unwrap(), "Kd");
        assert_eq!(card1.name().unwrap(), "king of diamonds");

        let card2 = Card::new_joker(JokerColor::Black);
        assert_eq!(card2.to_unicode().unwrap(), '🃏');
        assert_eq!(card2.to_suit_rank().unwrap(), "🃏2");
        assert_eq!(card2.to_ascii().unwrap(), "J2");
        assert_eq!(card2.name().unwrap(), "black joker");
    }
}
