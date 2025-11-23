use crate::game::Game;
use crate::dice::Dice;
use std::io::{self, Write};

pub struct UserInterface;

impl UserInterface {
    pub fn new() -> Self {
        UserInterface
    }
    
    pub fn show_rules(&self) {
        println!("\x1b[1;91m📜 GAME RULES:\x1b[0m");
        println!("─────────────────────────────────────────");
        println!("• Two players take turns rolling a dice");
        println!("• Each turn, you can roll multiple times");
        println!("• Add each roll to your turn total");
        println!("• BUT if you roll a 1, you lose all points for that turn!");
        println!("• You can 'hold' to bank your turn score");
        println!("• First player to reach 20 points wins! 🏆");
        println!("─────────────────────────────────────────\n");
    }
    
    pub fn display_game_state(&self, game: &Game) {
        println!("\n\x1b[1;96m╔═══════════════════════════════════════╗");
        println!("║            SCORE BOARD                ║");
        println!("╠═══════════════════════════════════════╣\x1b[0m");
        println!("║  Player 1: {:3} points                ║", game.get_player1_score());
        println!("║  Player 2: {:3} points                ║", game.get_player2_score());
        println!("\x1b[1;96m╠═══════════════════════════════════════╣\x1b[0m");
        println!("║  Current Player: Player {}            ║", game.get_current_player());
        println!("║  Turn Score: {:3} points              ║", game.get_turn_score());
        println!("\x1b[1;96m╚═══════════════════════════════════════╝\x1b[0m\n");
    }
    
    pub fn get_player_action(&self, game: &Game) -> String {
        print!("Player {}, choose action ([r]oll, [h]old, [q]uit): ", game.get_current_player());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_lowercase()
    }
    
    pub fn display_roll(&self, value: u8) {
        println!("\n🎲 You rolled:");
        println!("{}", Dice::get_ascii_art(value));
        
        if value == 1 {
            // Handled separately in display_pig_out
        } else {
            println!("✅ Added {} points to your turn score!", value);
        }
    }
    
    pub fn display_pig_out(&self) {
        println!("\n🐷 PIG OUT! You rolled a 1!");
        println!("💥 You lose all points from this turn!");
    }
    
    pub fn display_hold(&self, game: &Game, player: u8) {
        let score = if player == 1 { 
            game.get_player1_score() 
        } else { 
            game.get_player2_score() 
        };
        
        println!("\n💰 Player {} holds!", player);
        println!("🏦 Banked points! Total score: {}", score);
    }
    
    pub fn display_winner(&self, game: &Game) {
        if let Some(winner) = game.get_winner() {
            println!("\n");
            println!("\x1b[1;93m╔═══════════════════════════════════════════════════════════════════╗");
            println!("║                                                                   ║");
            println!("║  ██╗    ██╗██╗███╗   ██╗███╗   ██╗███████╗██████╗     ██╗       ║");
            println!("║  ██║    ██║██║████╗  ██║████╗  ██║██╔════╝██╔══██╗    ██║       ║");
            println!("║  ██║ █╗ ██║██║██╔██╗ ██║██╔██╗ ██║█████╗  ██████╔╝    ██║       ║");
            println!("║  ██║███╗██║██║██║╚██╗██║██║╚██╗██║██╔══╝  ██╔══██╗    ╚═╝       ║");
            println!("║  ╚███╔███╔╝██║██║ ╚████║██║ ╚████║███████╗██║  ██║    ██╗       ║");
            println!("║   ╚══╝╚══╝ ╚═╝╚═╝  ╚═══╝╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝    ╚═╝       ║");
            println!("║                                                                   ║");
            println!("║                       🏆 PLAYER {} 🏆                             ║", winner);
            println!("║                                                                   ║");
            println!("╚═══════════════════════════════════════════════════════════════════╝\x1b[0m");
            println!("\n\x1b[1;92m            🎉🎉🎉 CONGRATULATIONS! 🎉🎉🎉\x1b[0m\n");
        }
    }
}
