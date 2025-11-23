use crate::dice::Dice;

const WINNING_SCORE: u32 = 20;

pub struct Game {
    player1_score: u32,
    player2_score: u32,
    current_player: u8,
    turn_score: u32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player1_score: 0,
            player2_score: 0,
            current_player: 1,
            turn_score: 0,
        }
    }
    
    pub fn roll_dice(&mut self) -> u8 {
        let roll = Dice::roll();
        
        if roll == 1 {
            self.turn_score = 0;
        } else {
            self.turn_score += roll as u32;
        }
        
        roll
    }
    
    pub fn hold(&mut self) {
        if self.current_player == 1 {
            self.player1_score += self.turn_score;
        } else {
            self.player2_score += self.turn_score;
        }
        self.turn_score = 0;
    }
    
    pub fn next_turn(&mut self) {
        self.turn_score = 0;
        self.current_player = if self.current_player == 1 { 2 } else { 1 };
    }
    
    pub fn check_winner(&self) -> bool {
        self.player1_score >= WINNING_SCORE || self.player2_score >= WINNING_SCORE
    }
    
    pub fn check_winner_with_turn_score(&self) -> bool {
        let current_total = if self.current_player == 1 {
            self.player1_score + self.turn_score
        } else {
            self.player2_score + self.turn_score
        };
        current_total >= WINNING_SCORE
    }
    
    pub fn get_winner(&self) -> Option<u8> {
        if self.player1_score >= WINNING_SCORE {
            Some(1)
        } else if self.player2_score >= WINNING_SCORE {
            Some(2)
        } else {
            None
        }
    }
    
    pub fn get_player1_score(&self) -> u32 {
        self.player1_score
    }
    
    pub fn get_player2_score(&self) -> u32 {
        self.player2_score
    }
    
    pub fn get_current_player(&self) -> u8 {
        self.current_player
    }
    
    pub fn get_turn_score(&self) -> u32 {
        self.turn_score
    }
}
