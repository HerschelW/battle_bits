pub mod battle;
pub mod create_army;
pub mod warrior;

// use battle::battle;
use create_army::army;
fn main() {
    // start game
    start_game();
    // battle();
    // create_army(10);
}

// start game
pub fn start_game() {
    println!("Starting game...");
    // create army
    // army(10);
    // print army
    let army = army(10);
    println!("{:?}", army);
    // battle
    // battle(army);
}
