use crate::card::Card;
use crate::hand::Hand;

pub struct Strategy;

impl Strategy {
    pub fn get_optimal_play(hand: &Hand, dealer_card: &Card) -> &'static str {
        let player_value = hand.value();
        let dealer_value = dealer_card.value()[0];

        if hand.can_split() {
            return Self::get_split_decision(player_value, dealer_value);
        }

        Self::get_play_for_total(player_value, dealer_value)
    }
    fn get_split_decision(player_value: i32, dealer_value: i32) -> &'static str {
        match player_value {
            2 | 3 | 6 | 7 | 12 => if dealer_value <= 7 { "Split" } else { "Hit" },
            8 | 16 => "Split",  // Pairs of 4s, 8s
            18 => if dealer_value != 7 && dealer_value != 10 && dealer_value != 11 { "Split" } else { "Stand" },
            20 => "Stand",  // Pairs of 10s
            22 => "Split",  // Pairs of Aces
            _ => "Hit"
        }
    }

    pub fn get_play_for_total(player_value: i32, dealer_value: i32) -> &'static str {
        match player_value {
            2..11 => "Hit",
            12 => if dealer_value >= 4 && dealer_value <= 6 { "Stand" } else { "Hit" },
            13..=16 => if dealer_value <= 6 { "Stand" } else { "Hit" },
            _ => "Stand",
        }
    }
}