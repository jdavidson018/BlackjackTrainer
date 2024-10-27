use crate::card::Card;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Hand { cards }
    }

    pub fn value(&self) -> i32  {
        let mut values = vec![0];

        for card in &self.cards {
            let card_values = card.value();
            let mut new_values: Vec<i32> = Vec::new();

            for value in card_values {
                for curr in &values {
                    new_values.push(value+curr)
                }
            }
            values = new_values;
        }
        values.into_iter()
            .filter(|&x| x <= 21)
            .max()
            .unwrap_or(22)
    }

    pub fn can_split(&self) -> bool {
        (self.cards.len() == 2) && (self.cards[0].value() == self.cards[1].value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Card;
    #[test]
    fn test_hand_values() {
        let hand = Hand::new(vec![Card::Two, Card::Eight]);
        assert_eq!(hand.value(), 10);
    }

    #[test]
    fn test_can_split() {
        let can_same_card = Hand::new(vec![Card::Two, Card::Two]);
        let can_same_value = Hand::new(vec![Card::Ten, Card::King]);
        let cannot_diff_card= Hand::new(vec![Card::Two, Card::Ten]);
        let cannot_too_many_cards= Hand::new(vec![Card::Two, Card::Two, Card::Two]);

        assert!(can_same_card.can_split());
        assert!(can_same_value.can_split());
        assert!(!cannot_diff_card.can_split());
        assert!(!cannot_too_many_cards.can_split());
    }

}