use crate::cards::{Card, JokerColor, Rank, Suit};

pub fn generate_n_decks(n: usize, jokers: usize) -> Vec<Card> {
    if jokers > 3 {
        unimplemented!("Max number of jokers is 3.");
    }

    let mut vec = Vec::with_capacity(n * (52 + jokers));

    for _deck in 0..n {
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                vec.push(Card::new(suit.clone(), *rank));
            }
        }

        if jokers >= 1 {
            vec.push(Card::new_joker(JokerColor::Red));

            if jokers >= 2 {
                vec.push(Card::new_joker(JokerColor::Black));

                if jokers >= 3 {
                    vec.push(Card::new_joker(JokerColor::White));
                }
            }
        }
    }

    vec
}

pub fn generate_deck(jokers: usize) -> Vec<Card> {
    generate_n_decks(1, jokers)
}
