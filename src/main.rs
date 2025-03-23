// CRATES

use rand::prelude::*;
use rand::rngs::StdRng;
use std::collections::VecDeque;
use std::fmt;

// STRUCTS + METHODS

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn penalty_value(&self) -> Option<u8> {
        match self {
            Rank::Jack => Some(1),
            Rank::Queen => Some(2),
            Rank::King => Some(3),
            Rank::Ace => Some(4),
            _ => None,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rank::Two => "2",
                Rank::Three => "3",
                Rank::Four => "4",
                Rank::Five => "5",
                Rank::Six => "6",
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "10",
                Rank::Jack => "J",
                Rank::Queen => "Q",
                Rank::King => "K",
                Rank::Ace => "A",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Spade => "♤",
                Suit::Heart => "♡",
                Suit::Club => "♧",
                Suit::Diamond => "♢",
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    One,
    Two,
}

impl Player {
    fn other(&self) -> Self {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }

    fn number(&self) -> usize {
        match self {
            Player::One => 1,
            Player::Two => 2,
        }
    }
}

// LOGGING MODULE

mod logging {
    use super::{Card, Player};

    pub fn penalty_start(required: u8) {
        let rune: &str = match required {
            1 => "J",
            2 => "Q",
            3 => "K",
            _ => "A",
        };
        println!("\nNEW PENALTY PHASE: [{} - {}]\n", rune, required);
    }

    pub fn card_played(player: Player, card: &Card) {
        println!("\nPLAYER |{}| →  {}", player.number(), card);
    }

    pub fn cards_collected(player: Player, cards: &[Card]) {
        let cards_str = cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("\nPLAYER |{}| ←  [{}]", player.number(), cards_str);
        println!("\nEND PENALTY PHASE\n");
    }

    pub fn game_start() {
        println!("\n=== Game Start ===");
    }

    pub fn game_over(winner: Player) {
        println!("\n=== Game Over ===");
        println!("WINNER: PLAYER {}", winner.number());
    }

    pub fn turn_count(count: usize) {
        println!("\nTURNS: {}", count);
    }

    fn print_deck(deck: &[Card]) -> String {
        deck.iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn full_starting_deck(deck: &[Card]) {
        println!("\nINITIAL DECK ({}):", deck.len());
        println!("[{}]", print_deck(deck));
    }

    pub fn player_starting_deck(player: Player, deck: &[Card]) {
        println!(
            "\nPLAYER |{}| INITIAL DECK ({}):",
            player.number(),
            deck.len()
        );
        println!("[{}]", print_deck(deck));
    }
}

// HELPER FUNCTIONS

fn create_deck() -> Vec<Card> {
    let ranks = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];
    let suits = [Suit::Spade, Suit::Heart, Suit::Club, Suit::Diamond];

    let mut deck = Vec::with_capacity(52);
    for &suit in &suits {
        for &rank in &ranks {
            deck.push(Card { rank, suit });
        }
    }
    deck
}

fn process_penalty_phase(
    initial_player: Player,
    initial_penalty: u8,
    central_pile: &mut Vec<Card>,
    decks: &mut [VecDeque<Card>; 2],
) -> Option<Player> {
    let mut current_player = initial_player.other();
    let mut required = initial_penalty;
    let mut last_penalty_initiator = initial_player;

    logging::penalty_start(required);

    loop {
        let mut paid = 0;

        while paid < required {
            let player_idx = current_player as usize;

            if decks[player_idx].is_empty() {
                return Some(current_player.other());
            }

            let card = decks[player_idx].pop_front().unwrap();
            central_pile.push(card);
            paid += 1;

            logging::card_played(current_player, &card);

            if let Some(new_penalty) = card.rank.penalty_value() {
                logging::penalty_start(new_penalty);
                last_penalty_initiator = current_player;
                required = new_penalty;
                current_player = current_player.other();
                paid = 0;
                break;
            }
        }

        if paid == required {
            logging::cards_collected(last_penalty_initiator, central_pile);

            let target_idx = last_penalty_initiator as usize;
            decks[target_idx].extend(central_pile.drain(..));
            return None;
        }
    }
}

// MAIN GAME LOOP

fn main() {
    let seed = rand::rng().next_u64();
    println!("SEED: {}", seed);

    let mut rng = StdRng::seed_from_u64(seed);
    let mut deck = create_deck();
    deck.shuffle(&mut rng);

    let starting_deck = deck.iter().copied().collect::<Vec<_>>();
    let split_point = deck.len() / 2;
    let mut decks = [
        deck.drain(..split_point).collect::<VecDeque<_>>(),
        deck.drain(..).collect::<VecDeque<_>>(),
    ];

    let initial_decks = [
        decks[0].iter().copied().collect::<Vec<_>>(),
        decks[1].iter().copied().collect::<Vec<_>>(),
    ];

    let mut turn_count = 0;
    let mut current_player = Player::One;
    let mut central_pile = Vec::new();

    logging::full_starting_deck(&starting_deck);
    logging::player_starting_deck(Player::One, &initial_decks[0]);
    logging::player_starting_deck(Player::Two, &initial_decks[1]);

    logging::game_start();
    let winner = loop {
        let player_idx = current_player as usize;

        if decks[player_idx].is_empty() {
            break current_player.other();
        }

        let card = decks[player_idx].pop_front().unwrap();
        central_pile.push(card);

        logging::card_played(current_player, &card);

        if let Some(penalty) = card.rank.penalty_value() {
            if let Some(winner) =
                process_penalty_phase(current_player, penalty, &mut central_pile, &mut decks)
            {
                break winner;
            }
        } else {
            current_player = current_player.other();
            turn_count += 1;
        }

        if decks[0].is_empty() || decks[1].is_empty() {
            break if decks[0].is_empty() {
                Player::Two
            } else {
                Player::One
            };
        }
    };

    logging::game_over(winner);
    logging::turn_count(turn_count);
}
