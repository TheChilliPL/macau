use crate::macau::variant::MacauVariant;
use crate::macau::MacauGame;

mod cards;
mod macau;

fn main() {
    let game = MacauGame::new(MacauVariant::default(), vec!["Alice".into(), "Bob".into()]);

    println!("{:?}", game);
}
