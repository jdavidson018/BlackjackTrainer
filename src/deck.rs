use rand::seq::SliceRandom;
use crate::card::Card;

#[derive(Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let cards = vec![
            Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five, Card::Six,
            Card::Seven, Card::Eight, Card::Nine, Card::Ten, Card::Jack, Card::Queen,
            Card::King,
        ];

        let mut deck: Vec<Card> = Vec::with_capacity(52);
        for _ in 0..4 {
            deck.extend(cards.iter().cloned());
        }

        Deck { cards: deck }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn remaining(&self) -> usize { // usize is a pointer to an int
        self.cards.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deck_count() {
        let deck = Deck::new();
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_draw() {
        let mut deck = Deck::new();
        let card = deck.draw();
        assert!(card.is_some(), "Unable to draw a card");
        assert_eq!(deck.remaining(), 51, "Deck should be 1 less than 52")
    }

    #[test]
    fn test_shuffle() {
        let mut deck = Deck::new();
        let unshuffled = deck.clone();

        assert_eq!(deck.cards, unshuffled.cards);
        deck.shuffle();
        assert_ne!(deck.cards, unshuffled.cards);
    }

    #[test]
    fn test_empty_deck() {
        let mut deck = Deck::new();
        for _ in 0..52 {
            deck.draw();
        }
        assert_eq!(deck.remaining(), 0);
    }
}