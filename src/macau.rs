use crate::cards::deck::generate_deck;
use crate::cards::hand::{Hand, HasHand, SortedCard};
use crate::cards::pile::Pile;
use crate::macau::variant::MacauVariant;
use std::fmt::{Debug, Formatter};

pub mod variant;

pub struct MacauPlayer {
    name: String,
    hand: Hand<SortedCard>,
}

impl HasHand for MacauPlayer {
    type CardType = SortedCard;

    fn hand(&self) -> &Hand<Self::CardType> {
        &self.hand
    }

    fn hand_mut(&mut self) -> &mut Hand<Self::CardType> {
        &mut self.hand
    }
}

pub struct MacauGame {
    variant: MacauVariant,
    pile: Pile,
    players: Vec<MacauPlayer>,
}

impl MacauGame {
    pub fn new(variant: MacauVariant, player_names: Vec<String>) -> Self {
        let players = player_names
            .iter()
            .map(|name| MacauPlayer {
                name: name.clone(),
                hand: Hand::new(),
            })
            .collect();

        let mut game = MacauGame {
            variant,
            pile: Pile::of(generate_deck(3)),
            players,
        };

        for player in &mut game.players {
            for _ in 0..game.variant.initial_hand {
                player.deal(game.pile.pop().unwrap());
            }
        }

        game
    }
}

impl Debug for MacauGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // todo!();
        write!(
            f,
            "MacauGame {{\n\
            {}\n",
            self.pile,
        )?;
        for player in &self.players {
            write!(f, "{}: {}\n", player.name, player.hand)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
