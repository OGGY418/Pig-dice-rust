use rand::Rng;

pub struct Dice;

impl Dice {
    pub fn roll() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=6)
    }
    
    pub fn get_ascii_art(value: u8) -> String {
        match value {
            1 => String::from(
                "┌─────────┐\n\
                 │         │\n\
                 │    ●    │\n\
                 │         │\n\
                 └─────────┘"
            ),
            2 => String::from(
                "┌─────────┐\n\
                 │  ●      │\n\
                 │         │\n\
                 │      ●  │\n\
                 └─────────┘"
            ),
            3 => String::from(
                "┌─────────┐\n\
                 │  ●      │\n\
                 │    ●    │\n\
                 │      ●  │\n\
                 └─────────┘"
            ),
            4 => String::from(
                "┌─────────┐\n\
                 │  ●   ●  │\n\
                 │         │\n\
                 │  ●   ●  │\n\
                 └─────────┘"
            ),
            5 => String::from(
                "┌─────────┐\n\
                 │  ●   ●  │\n\
                 │    ●    │\n\
                 │  ●   ●  │\n\
                 └─────────┘"
            ),
            6 => String::from(
                "┌─────────┐\n\
                 │  ●   ●  │\n\
                 │  ●   ●  │\n\
                 │  ●   ●  │\n\
                 └─────────┘"
            ),
            _ => String::from("Invalid dice value")
        }
    }
}
