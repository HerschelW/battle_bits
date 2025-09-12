pub mod modules {
    pub mod battle;
    pub mod battle_enhanced;
    pub mod combat_calculator;
    pub mod constants;
    pub mod create_army;
    pub mod visualization;
    pub mod warrior;
    pub mod warrior_enhanced;
}

use modules::battle::battle;
use modules::create_army::army;
use modules::warrior::Warrior;

fn main() {
    println!("🎮 Welcome to BATTLE BITS! 🎮");
    println!("Rock, Paper, Scissors warriors clash in epic battles!\n");

    let victorious_army1 = start_game();
    let victorious_army2 = start_game();
    let enemy_army1 = army(500);
    let enemy_army2 = army(500);

    println!("\n🏆 CHAMPIONSHIP ROUNDS 🏆");
    println!("The victorious armies now face new challengers!\n");

    let champion_army1 = battle(victorious_army1, enemy_army1);
    println!(
        "\nChampion Army 1: {} warriors remain",
        champion_army1.len()
    );
    let champion_army2 = battle(victorious_army2, enemy_army2);
    println!(
        "\nChampion Army 2: {} warriors remain",
        champion_army2.len()
    );

    println!("\n🥇 FINAL BATTLE 🥇");
    println!("The ultimate showdown begins!\n");

    let winner = battle(champion_army1, champion_army2);

    println!("\n🎉 ULTIMATE CHAMPION! 🎉");
    if let Some(champion) = winner.first() {
        println!("The ultimate warrior: {}", champion.name);
        println!("Type: {:?}", champion.warrior_type);
        println!(
            "Final Stats - Health: {}, Attack: {}, Defense: {}",
            champion.health, champion.attack_modifier, champion.defense_modifier
        );
    }
    println!("Total surviving warriors: {}", winner.len());
}

pub fn start_game() -> Vec<Warrior> {
    println!("⚔️  Battle begins! ⚔️");
    let player_army = army(500);
    let enemy_army = army(500);
    println!("Player army count: {}", player_army.len());
    println!("Enemy army count: {}", enemy_army.len());
    let victorious_army = battle(player_army, enemy_army);
    println!("Victorious army: {} warriors remain", victorious_army.len());
    return victorious_army;
}
