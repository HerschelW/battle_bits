pub mod modules {
  pub mod battle;
  pub mod create_army;
  pub mod warrior;
}

use modules::battle::battle;
use modules::create_army::army;
use modules::warrior::Warrior;

fn main() {
  let victorious_army1 = start_game();
  let victorious_army2 = start_game();
  let enemy_army1 = army(500);
  let enemy_army2 = army(500);

//   println!(
//     "Victorious army 2 hero:\n {},\n health: {},\n attack: {},\n defense: {} \n\n\n",
//     victorious_army2[0].name,
//     victorious_army2[0].health,
//     victorious_army2[0].attack_modifier,
//     victorious_army2[0].defense_modifier
//   );
//   println!("\n\n\n");

  let champion_army1 = battle(victorious_army1, enemy_army1);
  println!("Champion Army 1: {:?}", champion_army1);
  let champion_army2 = battle(victorious_army2, enemy_army2);
  println!("Champion Army 2: {:?}", champion_army2);

  let winner = battle(champion_army1, champion_army2);
  println!("Winner: {:?}", winner);
}

pub fn start_game() -> Vec<Warrior> {
  println!("Starting game...");
  let player_army = army(500);
  let enemy_army = army(500);
  println!("Player army count: {}", player_army.len());
  println!("Enemy army count: {}", enemy_army.len());
  let victorious_army = battle(player_army, enemy_army);
  return victorious_army;
}
