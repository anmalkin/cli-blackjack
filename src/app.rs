use crate::cards::*;

const ACE_HIGH: u8 = 11;
const ACE_LOW: u8 = 1;
const BLACKJACK: u8 = 21;
const FACECARD: u8 = 10;
const DEALER_STAND: u8 = 17;

#[derive(Debug)]
pub struct App {
    pub bank: u32,
    pub player_hand: Vec<Card>,
    pub dealer_hand: Vec<Card>,
    pub current_bet: u32,
    pub blackjack_payout: u32,
    pub state: GameState,
}

impl App {
    pub fn new(bank: u32) -> Self {
        let player_hand = Vec::new();
        let dealer_hand = Vec::new();

        App {
            bank,
            player_hand,
            dealer_hand,
            current_bet: 0,
            blackjack_payout: 0,
            state: GameState::EnterBet,
        }
    }

    pub fn place_bet(&mut self, bet: u32) {
        self.current_bet = bet;
        self.blackjack_payout = bet * 3 / 2;
        self.state = GameState::PlayerTurn;
    }

    pub fn start(&mut self) {
        self.player_hand = vec![Card::new(), Card::new()];
        self.dealer_hand = vec![Card::new(), Card::new()];
        self.dealer_hand[0].face_down();
        if calc_hand_score(&self.player_hand) == BLACKJACK {
            self.state = GameState::Blackjack;
            self.bank += self.blackjack_payout;
        } else {
            self.state = GameState::PlayerTurn;
        }
    }

    pub fn reset(&mut self) {
        self.current_bet = 0;
        self.player_hand.clear();
        self.dealer_hand.clear();
        self.state = GameState::EnterBet;
    }

    pub fn player_score(&self) -> u8 {
        calc_hand_score(&self.player_hand)
    }

    pub fn dealer_showing(&self) -> u8 {
        calc_card_score(&self.dealer_hand[1])
    }

    pub fn dealer_score(&self) -> u8 {
        calc_hand_score(&self.dealer_hand)
    }

    pub fn run(&mut self, command: Command) {
        match command {
            Command::Hit => {
                self.player_hand.push(Card::new());
                if self.player_score() > BLACKJACK {
                    self.state = GameState::Lose;
                    self.bank -= self.current_bet;
                }
                if self.player_score() == BLACKJACK {
                    self.state = GameState::DealerTurn;
                    self.flip_upcard();
                }
            }
            Command::Stand => {
                self.state = GameState::DealerTurn;
                self.flip_upcard();
            },
            Command::AdvanceDealer => {
                if self.dealer_score() < DEALER_STAND {
                    self.dealer_hand.push(Card::new());
                } else if self.dealer_score() > BLACKJACK
                    || self.dealer_score() < self.player_score()
                {
                    // Ensure dealer does not run after player has already lost
                    assert!(self.player_score() <= BLACKJACK);
                    self.state = GameState::Win;
                    self.bank += self.current_bet;
                } else if self.dealer_score() == self.player_score() {
                    self.state = GameState::Draw;
                } else {
                    self.state = GameState::Lose;
                    self.bank -= self.current_bet;
                }
            }
            Command::Split => todo!(),
        }
    }

    fn flip_upcard(&mut self) {
        self.dealer_hand[0].face_up();
    }
}

/// Default bank amount set to $100
impl Default for App {
    fn default() -> Self {
        App::new(100)
    }
}

#[derive(Debug)]
pub enum GameState {
    EnterBet,
    PlayerTurn,
    DealerTurn,
    Win,
    Lose,
    Blackjack,
    Draw,
}

#[derive(Debug)]
pub enum Command {
    Hit,
    Stand,
    AdvanceDealer,
    Split,
}

/// Calculate current score of blackjack hand. Aces are scored as 11 unless the total score is
/// above 21, in which case they are scored as 1.
fn calc_hand_score(hand: &[Card]) -> u8 {
    let mut aces = 0;
    let mut score = 0;
    for card in hand {
        if let Rank::Ace = card.rank {
            aces += 1;
        }
        score += calc_card_score(card);
    }

    // Adjust Aces value downward if necessary
    while score > BLACKJACK && aces > 0 {
        score -= ACE_HIGH - ACE_LOW; // note operator precedence
        aces -= 1;
        assert!(score >= 2);
    }
    score
}

fn calc_card_score(card: &Card) -> u8 {
    match card.rank {
        Rank::Ace => ACE_HIGH,
        Rank::Pip(num) => num,
        _ => FACECARD,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn deal() {
        let mut app = App::default();
        app.start();
        let player_count = app.player_hand.len();
        let dealer_count = app.dealer_hand.len();
        assert_eq!(2, player_count);
        assert_eq!(2, dealer_count);
        assert!(app.player_score() > 1);
        assert!(app.dealer_showing() > 1);
    }

    #[test]
    fn hit() {
        let mut app = App::default();
        app.start();
        app.run(Command::Hit);
        let player_count = app.player_hand.len();
        let dealer_count = app.dealer_hand.len();
        assert_eq!(3, player_count);
        assert_eq!(2, dealer_count);
    }

    #[test]
    fn stand() {
        let mut app = App::default();
        app.start();
        let old_player_score = app.player_score();
        app.run(Command::Stand);
        assert_eq!(old_player_score, app.player_score());
        matches!(app.state, GameState::Win | GameState::Lose);
    }

    #[test]
    fn calc_score_test() {
        let jack_of_spades = Card {
            suit: Suit::Spades,
            rank: Rank::Jack,
            down: false,
        };
        let two_of_diamonds = Card {
            suit: Suit::Diamonds,
            rank: Rank::Pip(2),
            down: false,
        };
        let hand = vec![jack_of_spades, two_of_diamonds];
        assert_eq!(calc_hand_score(&hand), 12);

        let ace_of_hearts = Card {
            suit: Suit::Hearts,
            rank: Rank::Ace,
            down: false,
        };
        let king_of_diamonds = Card {
            suit: Suit::Diamonds,
            rank: Rank::King,
            down: false,
        };
        let hand = vec![ace_of_hearts, king_of_diamonds];
        assert_eq!(calc_hand_score(&hand), 21);

        let ace_of_hearts = Card {
            suit: Suit::Hearts,
            rank: Rank::Ace,
            down: false,
        };
        let ace_of_spades = Card {
            suit: Suit::Spades,
            rank: Rank::Ace,
            down: false,
        };
        let hand = vec![ace_of_hearts, ace_of_spades];
        assert_eq!(calc_hand_score(&hand), 12);

        let three_of_hearts = Card {
            suit: Suit::Hearts,
            rank: Rank::Pip(3),
            down: false,
        };
        let four_of_clubs = Card {
            suit: Suit::Hearts,
            rank: Rank::Pip(4),
            down: false,
        };
        let hand = vec![three_of_hearts, four_of_clubs];
        assert_eq!(calc_hand_score(&hand), 7);

        // Ensure scoring logic for aces is working appropriately
        let mut cards: Vec<Card> = Vec::new();
        for _ in 1..13 {
            cards.push(Card {
                suit: Suit::Hearts,
                rank: Rank::Ace,
            down: false,
            })
        }
        assert_eq!(calc_hand_score(&cards), 12);
    }
}
