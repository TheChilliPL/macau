use crate::cards::hand::HasHand;
use crate::cards::Card;
use std::fmt;
use std::fmt::Display;

pub struct Pile {
    cards: Vec<Card>,
    accessible: usize,
}

impl Pile {
    pub fn new_empty() -> Self {
        Pile {
            cards: Vec::new(),
            accessible: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Pile {
            cards: Vec::with_capacity(capacity),
            accessible: 0,
        }
    }

    pub fn of(cards: Vec<Card>) -> Self {
        Pile {
            cards,
            accessible: 0,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_cards(&mut self, cards: Vec<Card>) {
        self.cards.extend(cards);
    }

    pub fn add_on_top(&mut self, card: Card) {
        self.cards.push(card);
        let last = self.cards.len() - 1;
        self.cards.swap(self.accessible, last);
        self.accessible += 1;
    }

    pub fn pop(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            return None;
        }
        if self.accessible <= 0 {
            self.shuffle();
        }
        self.accessible -= 1;
        Some(self.cards.swap_remove(self.accessible))
    }

    pub fn seek(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            return None;
        }
        if self.accessible <= 0 {
            self.shuffle();
        }
        Some(self.cards[self.accessible - 1])
    }

    pub fn try_seek(&self) -> Option<Card> {
        if self.cards.is_empty() {
            return None;
        }
        if self.accessible <= 0 {
            return None;
        }
        Some(self.cards[self.accessible - 1])
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        self.cards.shuffle(&mut thread_rng());

        self.accessible = self.cards.len();
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn count_accessible(&self) -> usize {
        self.accessible
    }

    pub fn count_total(&self) -> usize {
        self.cards.len()
    }

    pub fn deal_to<C: Ord + From<Card> + Into<Card> + Clone>(
        &mut self,
        other: &mut dyn HasHand<CardType = C>,
    ) -> Result<(), PileEmptyError> {
        if let Some(card) = self.pop() {
            other.deal(card);
            Ok(())
        } else {
            Err(PileEmptyError)
        }
    }
}

impl Display for Pile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.cards.is_empty() {
            return write!(f, "Empty pile");
        }
        write!(
            f,
            "Pile with {} cards, {} on top",
            self.cards.len(),
            self.try_seek()
                .map(|c| c.to_suit_rank().unwrap())
                .unwrap_or_else(|| "unknown".to_string())
        )
    }
}

#[derive(Debug, Clone)]
pub struct PileEmptyError;

impl Display for PileEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pile is empty.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Rank, Suit};

    #[allow(unused_variables)]
    #[test]
    fn pile_test() {
        let a = Card::new(Suit::Hearts, Rank::Ace);
        let b = Card::new(Suit::Hearts, Rank::Two);
        let c = Card::new(Suit::Hearts, Rank::Three);
        let d = Card::new(Suit::Hearts, Rank::Four);
        let e = Card::new(Suit::Hearts, Rank::Five);
        let f = Card::new(Suit::Hearts, Rank::Six);

        let mut pile = Pile::of(vec![a, b, c, d]);
        assert_eq!(pile.count_total(), 4);
        assert_eq!(pile.count_accessible(), 0);
        // Shuffle the pile
        pile.shuffle();
        assert_eq!(pile.count_total(), 4);
        assert_eq!(pile.count_accessible(), 4);

        let (a, b, c, d) = (pile.cards[0], pile.cards[1], pile.cards[2], pile.cards[3]);
        assert_eq!(pile.seek(), Some(d));
        let popped = pile.pop();
        assert_eq!(popped, Some(d));
        assert_eq!(pile.count_total(), 3);
        assert_eq!(pile.count_accessible(), 3);
        assert_eq!(pile.seek(), Some(c));

        pile.add_card(e);
        assert_eq!(pile.count_total(), 4);
        assert_eq!(pile.count_accessible(), 3);
        assert_eq!(pile.seek(), Some(c));

        pile.add_on_top(f);
        assert_eq!(pile.count_total(), 5);
        assert_eq!(pile.count_accessible(), 4);
        assert_eq!(pile.seek(), Some(f));
    }
}
