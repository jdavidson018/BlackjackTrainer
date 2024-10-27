#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Card {
    Two, Three, Four, Five, Six, Seven, Eight,
    Nine, Ten, Jack, Queen, King, Ace,
}

impl Card {
    pub fn value(&self) -> Vec<i32> {
        match self {
            Card::Ace => vec![1, 11],
            Card::Two => vec![2],
            Card::Three => vec![3],
            Card::Four => vec![4],
            Card::Five => vec![5],
            Card::Six => vec![6],
            Card::Seven => vec![7],
            Card::Eight => vec![8],
            Card::Nine => vec![9],
            Card::Ten | Card::Jack | Card::Queen | Card::King => vec![10],
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Card::Ace => "A".to_string(),
            Card::Two => "2".to_string(),
            Card::Three => "3".to_string(),
            Card::Four => "4".to_string(),
            Card::Five => "5".to_string(),
            Card::Six => "6".to_string(),
            Card::Seven => "7".to_string(),
            Card::Eight => "8".to_string(),
            Card::Nine => "9".to_string(),
            Card::Ten => "10".to_string(),
            Card::Jack => "J".to_string(),
            Card::Queen => "Q".to_string(),
            Card::King => "K".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_values() {
        assert_eq!(Card::Ace.value(), vec![1, 11]);
        assert_eq!(Card::Two.value(), vec![2]);
        assert_eq!(Card::Three.value(), vec![3]);
        assert_eq!(Card::Four.value(), vec![4]);
        assert_eq!(Card::Five.value(), vec![5]);
        assert_eq!(Card::Six.value(), vec![6]);
        assert_eq!(Card::Seven.value(), vec![7]);
        assert_eq!(Card::Eight.value(), vec![8]);
        assert_eq!(Card::Nine.value(), vec![9]);
        assert_eq!(Card::Ten.value(), vec![10]);
        assert_eq!(Card::Jack.value(), vec![10]);
        assert_eq!(Card::Queen.value(), vec![10]);
        assert_eq!(Card::King.value(), vec![10]);
    }

    #[test]
    fn test_card_to_string() {
        assert_eq!(Card::Ace.to_string(), "A");
        assert_eq!(Card::Two.to_string(), "2");
        assert_eq!(Card::Three.to_string(), "3");
        assert_eq!(Card::Four.to_string(), "4");
        assert_eq!(Card::Five.to_string(), "5");
        assert_eq!(Card::Six.to_string(), "6");
        assert_eq!(Card::Seven.to_string(), "7");
        assert_eq!(Card::Eight.to_string(), "8");
        assert_eq!(Card::Nine.to_string(), "9");
        assert_eq!(Card::Ten.to_string(), "10");
        assert_eq!(Card::Jack.to_string(), "J");
        assert_eq!(Card::Queen.to_string(), "Q");
        assert_eq!(Card::King.to_string(), "K");
    }
}
