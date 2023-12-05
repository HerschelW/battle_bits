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

  println!(
    "Victorious army 1 hero:\n {},\n health: {},\n attack: {},\n defense: {} \n\n\n",
    victorious_army1[0].name,
    victorious_army1[0].health,
    victorious_army1[0].attack_modifier,
    victorious_army1[0].defense_modifier
  );

  println!(
    "Victorious army 2 hero:\n {},\n health: {},\n attack: {},\n defense: {} \n\n\n",
    victorious_army2[0].name,
    victorious_army2[0].health,
    victorious_army2[0].attack_modifier,
    victorious_army2[0].defense_modifier
  );
  println!("\n\n\n");

  let champion_army = battle(victorious_army1, victorious_army2);
  println!("Champion: {:?}", champion_army[0]);
}

pub fn start_game() -> Vec<Warrior> {
  println!("Starting game...");
  let player_army = army(100);
  let enemy_army = army(100);
  println!("Player army count: {}", player_army.len());
  println!("Enemy army count: {}", enemy_army.len());
  let victorious_army = battle(player_army, enemy_army);
  return victorious_army;
}
