use crate::cards::Card;
use sorted_vec::SortedVec;
use std::fmt;

/// Standard wrapper for a card that implements [Ord] and [PartialOrd].
///
/// This is used to sort cards in a hand. The sorting order is:
/// 1. Aces to kings of clubs.
/// 2. Aces to kings of diamonds.
/// 3. Aces to kings of hearts.
/// 4. Aces to kings of spades.
/// 5. Jokers, in the order red, black, white.
///
/// This isn't necessarily the only way to sort cards, which is why this is a separate type,
/// and [Card] itself doesn't implement [Ord] or [PartialOrd].
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SortedCard(Card);

impl From<Card> for SortedCard {
    fn from(card: Card) -> Self {
        SortedCard(card)
    }
}

impl Into<Card> for SortedCard {
    fn into(self) -> Card {
        self.0
    }
}

impl Ord for SortedCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0 == other.0 {
            return std::cmp::Ordering::Equal;
        }

        // Two jokers
        if self.0.is_joker() && other.0.is_joker() {
            return (self.0.joker_color().unwrap() as u8)
                .cmp(&(other.0.joker_color().unwrap() as u8));
        }

        // One joker
        if self.0.is_joker() {
            return std::cmp::Ordering::Greater;
        } else if other.0.is_joker() {
            return std::cmp::Ordering::Less;
        }

        // Standard cards
        let self_suit = self.0.suit().unwrap();
        let other_suit = other.0.suit().unwrap();
        let self_rank = self.0.rank().unwrap();
        let other_rank = other.0.rank().unwrap();

        if self_suit == other_suit {
            self_rank.cmp(&other_rank)
        } else {
            (self_suit as u8).cmp(&(other_suit as u8))
        }
    }
}

impl PartialOrd for SortedCard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Represents a hand of cards.
///
/// It's always sorted in the order defined by `T`.
/// If unsure, use [SortedCard] as `T`.
#[derive(Debug)]
pub struct Hand<T: Ord> {
    cards: SortedVec<T>,
}

impl<T: Ord + From<Card> + Into<Card> + Clone> Hand<T> {
    pub fn new() -> Self {
        Hand {
            cards: SortedVec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Hand {
            cards: SortedVec::with_capacity(capacity),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.insert(card.into());
    }

    pub fn remove_card(&mut self, card: Card) {
        self.cards.remove_item(&card.into());
    }

    pub fn cards(&self) -> &SortedVec<T> {
        &self.cards
    }

    pub fn iter(&self) -> impl Iterator<Item = Card> + '_ {
        self.cards.iter().cloned().map(|card| card.into())
    }
}

impl<T: Ord + From<Card> + Into<Card> + Clone> fmt::Display for Hand<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cards: Vec<String> = self
            .iter()
            .map(|card| card.to_suit_rank().unwrap_or_else(|()| "??".to_string()))
            .collect();
        write!(f, "{}", cards.join(", "))
    }
}

pub trait HasHand {
    type CardType: Ord + From<Card> + Into<Card> + Clone;

    fn hand(&self) -> &Hand<Self::CardType>;
    fn hand_mut(&mut self) -> &mut Hand<Self::CardType>;

    fn deal(&mut self, card: Card) {
        self.hand_mut().add_card(card);
    }
}
