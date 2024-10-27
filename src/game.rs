use crate::deck::Deck;
use crate::stats::Stats;
use std::io;
use std::io::Write;
use crate::card::Card;
use crate::hand::Hand;
use crate::strategy::Strategy;

pub struct Game {
    deck: Deck,
    stats: Stats,
}

impl Game {
    pub fn new() -> Self {
        Game {
            deck: Deck::new(),
            stats: Stats::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Jack's Blackjack Trainer!");
        println!("Enter: (h)it, (s)tand, (d)ouble, s(p)lit");
        println!("Press 'q' to quit\n");

        while self.play_round() {}
        self.print_final_stats();
    }

    fn play_round(&mut self) -> bool {
        self.deck.shuffle();
        let hand = Hand::new(vec![
            self.deck.draw().unwrap(),
            self.deck.draw().unwrap()
        ]);
        let dealer_card = self.deck.draw().unwrap();

        println!("\nYour cards: {:?}", hand);
        println!("Dealer shows: {:?}", dealer_card);

        match self.get_player_input() {
            Some(play) => {
                self.handle_play(&hand, &dealer_card, &play);
                true
            }
            None => false
        }
    }

    fn get_player_input(&self) -> Option<String> {
        println!("Your play?");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim().to_lowercase();
        if input == "q" || input == "quit" {
            None
        } else {
            Some(input)
        }
    }

    fn handle_play(&mut self, hand: &Hand, dealer_card: &Card, play: &str) {
        let player_decision = match play {
            "h" => "Hit",
            "s" => "Stand",
            "d" => "Double if allowed, otherwise Hit",
            "p" => "Split",
            _ => {
                println!("Invalid input! Use h, s, d, or p");
                return;
            }
        };

        let correct_play = Strategy::get_optimal_play(hand, dealer_card);
        let is_correct = player_decision == correct_play;

        self.stats.record_result(is_correct);

        if is_correct {
            println!("Correct! ✓");
        } else {
            println!("Incorrect. Optimal play: {}", correct_play);
        }

        println!("Current score: {:.1}%", self.stats.get_percentage_correct());
    }

    fn print_final_stats(&self) {
        println!("\nFinal Score: {:.1}%", self.stats.get_percentage_correct());
        println!("Total hands: {}", self.stats.total_questions);
    }
}
