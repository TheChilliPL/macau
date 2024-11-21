use crate::cards::{Card, Rank, Suit};

#[derive(Debug, Clone)]
pub struct MacauVariant {
    pub initial_hand: u8,
    pub cumulate_war: bool,
    pub war_king_of_spades: u8,
    pub war_king_of_hearts: u8,
    pub war_king_of_diamonds: u8,
    pub war_king_of_clubs: u8,
    pub cumulate_blocks: bool,
    pub queen_of_spades_on_everything: bool,
    pub queen_of_hearts_on_everything: bool,
    pub queen_of_diamonds_on_everything: bool,
    pub queen_of_clubs_on_everything: bool,
    pub override_jack: bool,
    pub override_ace: bool,
}

impl Default for MacauVariant {
    fn default() -> Self {
        MacauVariant {
            initial_hand: 5,
            cumulate_war: true,
            war_king_of_spades: 5,
            war_king_of_hearts: 5,
            war_king_of_diamonds: 0,
            war_king_of_clubs: 0,
            cumulate_blocks: true,
            queen_of_spades_on_everything: true,
            queen_of_hearts_on_everything: true,
            queen_of_diamonds_on_everything: false,
            queen_of_clubs_on_everything: false,
            override_jack: true,
            override_ace: true,
        }
    }
}

impl MacauVariant {
    pub fn get_war_value(&self, card: Card) -> u8 {
        match card.rank() {
            Some(Rank::Two) => 2,
            Some(Rank::Three) => 3,
            Some(Rank::King) => match card.suit() {
                Some(Suit::Spades) => self.war_king_of_spades,
                Some(Suit::Hearts) => self.war_king_of_hearts,
                Some(Suit::Diamonds) => self.war_king_of_diamonds,
                Some(Suit::Clubs) => self.war_king_of_clubs,
                _ => 0,
            },
            _ => 0,
        }
    }

    pub fn is_war_card(&self, card: Card) -> bool {
        self.get_war_value(card) > 0
    }

    pub fn is_action_card(&self, card: Card) -> bool {
        match card.rank() {
            Some(Rank::Ace) => true,
            Some(Rank::Two) => true,
            Some(Rank::Three) => true,
            Some(Rank::Four) => true,
            Some(Rank::Jack) => true,
            Some(Rank::Queen) => match card.suit() {
                Some(Suit::Spades) => self.queen_of_spades_on_everything,
                Some(Suit::Hearts) => self.queen_of_hearts_on_everything,
                Some(Suit::Diamonds) => self.queen_of_diamonds_on_everything,
                Some(Suit::Clubs) => self.queen_of_clubs_on_everything,
                _ => false,
            },
            Some(Rank::King) => self.get_war_value(card) > 0,
            _ if card.is_joker() => true,
            _ => false,
        }
    }
}
