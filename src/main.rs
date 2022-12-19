pub mod battle;
pub mod create_army;

use battle::battle;
use create_army::create_army;
fn main() {
    // battle();
    // create_army(10);
}

// start game
pub fn start_game() {
    // create army
    create_army(10);
    // battle
    // battle();
    // print army
    println!("{:?}", create_army::warriors::new())
}
