use crate::cards::deck::generate_deck;
use crate::cards::hand::{Hand, HasHand, SortedCard};
use crate::cards::pile::Pile;
use crate::cards::Card;
use crate::macau::events::EventManager;
use crate::macau::variant::MacauVariant;
use std::fmt::{Debug, Formatter};

mod events;
pub mod variant;

#[derive(Debug)]
pub struct MacauPlayer {
    pub id: u32,
    pub name: String,
    pub hand: Hand<SortedCard>,
}

impl PartialEq for MacauPlayer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for MacauPlayer {}

impl HasHand for MacauPlayer {
    type CardType = SortedCard;

    fn hand(&self) -> &Hand<Self::CardType> {
        &self.hand
    }

    fn hand_mut(&mut self) -> &mut Hand<Self::CardType> {
        &mut self.hand
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MacauAction<'a> {
    Play(Card),
    PlayMultiple(&'a [Card]),
    Draw,
    DeclareMacau,
    Pass,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameEndReason<'a> {
    PlayerWon(&'a MacauPlayer),
    NotEnoughPlayers,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MacauEvent<'a> {
    GameStart {
        players: &'a [MacauPlayer],
        top_card: Card,
        your_cards: &'a [SortedCard],
    },
    TurnStart {
        player: &'a MacauPlayer,
    },
    TurnBlocked {
        player: &'a MacauPlayer,
    },
    PlayerAction {
        player: &'a MacauPlayer,
        action: MacauAction<'a>,
    },
    TurnEnd {
        player: &'a MacauPlayer,
    },
    AddCards {
        player: &'a MacauPlayer,
        cards: &'a [Card],
    },
    GameEnd {
        reason: GameEndReason<'a>,
    },
}

pub struct MacauGame {
    variant: MacauVariant,
    pile: Pile,
    players: Vec<MacauPlayer>,
    event_manager: EventManager,
}

impl MacauGame {
    pub fn new(variant: MacauVariant, player_names: Vec<String>) -> Self {
        let players = player_names
            .iter()
            .map(|name| MacauPlayer {
                id: rand::random(),
                name: name.clone(),
                hand: Hand::new(),
            })
            .collect();

        let mut game = MacauGame {
            variant,
            pile: Pile::of(generate_deck(3)),
            players,
            event_manager: EventManager::new(),
        };

        for player in &mut game.players {
            for _ in 0..game.variant.initial_hand {
                player.deal(game.pile.pop().unwrap());
            }
        }

        let top_card = game.pile.seek().unwrap();

        let event_manager = &game.event_manager;
        event_manager.notify_customized(&game, |game, id| MacauEvent::GameStart {
            players: &game.players,
            top_card,
            your_cards: game.get_player_by_id(id).unwrap().hand().cards(),
        });

        game
    }

    fn get_player_by_id(&self, id: u32) -> Option<&MacauPlayer> {
        self.players.iter().find(|player| player.id == id)
    }

    fn get_player_by_id_mut(&mut self, id: u32) -> Option<&mut MacauPlayer> {
        self.players.iter_mut().find(|player| player.id == id)
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
