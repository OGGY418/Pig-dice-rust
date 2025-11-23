mod dice;
mod game;
mod user_interface;

use game::Game;
use user_interface::UserInterface;

fn main() {
    println!("\x1b[1;96m"); // Bright Cyan (Sky Blue) with bold
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║                                                       ║");
    println!("║         ██████╗ ██╗ ██████╗                          ║");
    println!("║         ██╔══██╗██║██╔════╝                          ║");
    println!("║         ██████╔╝██║██║  ███╗                         ║");
    println!("║         ██╔═══╝ ██║██║   ██║                         ║");
    println!("║         ██║     ██║╚██████╔╝                         ║");
    println!("║         ╚═╝     ╚═╝ ╚═════╝                          ║");
    println!("║                                                       ║");
    println!("║     ██████╗ ██╗ ██████╗███████╗                      ║");
    println!("║     ██╔══██╗██║██╔════╝██╔════╝                      ║");
    println!("║     ██║  ██║██║██║     █████╗                        ║");
    println!("║     ██║  ██║██║██║     ██╔══╝                        ║");
    println!("║     ██████╔╝██║╚██████╗███████╗                      ║");
    println!("║     ╚═════╝ ╚═╝ ╚═════╝╚══════╝                      ║");
    println!("║                                                       ║");
    println!("║                  🎲  GAME  🎲                         ║");
    println!("║                                                       ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!("\x1b[0m");
    println!();
    
    let mut ui = UserInterface::new();
    let mut game = Game::new();
    
    ui.show_rules();
    
    loop {
        ui.display_game_state(&game);
        
        let action = ui.get_player_action(&game);
        
        match action.as_str() {
            "r" | "roll" => {
                let roll = game.roll_dice();
                ui.display_roll(roll);
                
                if roll == 1 {
                    ui.display_pig_out();
                    game.next_turn();
                } else {
                    // Check if rolling made them win (total + turn score >= 20)
                    if game.check_winner_with_turn_score() {
                        println!("\n🎊 You've reached the winning score!");
                        println!("\nPress Enter to see the results...");
                        let mut _buffer = String::new();
                        std::io::stdin().read_line(&mut _buffer).unwrap();
                        
                        let current = game.get_current_player();
                        game.hold(); // Bank the winning score
                        ui.display_winner(&game);
                        break;
                    }
                }
            }
            "h" | "hold" => {
                let current = game.get_current_player();
                game.hold();
                ui.display_hold(&game, current);
                
                // Check for winner immediately after holding
                if game.check_winner() {
                    println!("\nPress Enter to see the results...");
                    let mut _buffer = String::new();
                    std::io::stdin().read_line(&mut _buffer).unwrap();
                    ui.display_winner(&game);
                    break;
                }
                
                game.next_turn();
            }
            "q" | "quit" => {
                println!("\n👋 Thanks for playing!");
                break;
            }
            _ => {
                println!("❌ Invalid input! Use 'r' to roll or 'h' to hold.");
            }
        }
        
        println!("\nPress Enter to continue...");
        let mut _buffer = String::new();
        std::io::stdin().read_line(&mut _buffer).unwrap();
    }
}
