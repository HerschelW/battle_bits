pub mod modules {
    pub mod battle;
    pub mod battle_enhanced;
    pub mod constants;
    pub mod create_army;
    pub mod visualization;
    pub mod warrior;
}

use modules::battle_enhanced::battle_enhanced;
use modules::constants::*;
use modules::create_army::army;
use modules::warrior::Warrior;

fn main() {
    println!("🎮 Welcome to BATTLE BITS - Enhanced Edition! 🎮");
    println!("Rock, Paper, Scissors warriors clash in epic battles!\n");

    // Create armies with different sizes for variety
    let army_size = DEFAULT_ARMY_SIZE;
    println!("Creating armies of {} warriors each...", army_size);

    let victorious_army1 = start_enhanced_game("Player 1", army_size);
    let victorious_army2 = start_enhanced_game("Player 2", army_size);
    let enemy_army1 = army(army_size);
    let enemy_army2 = army(army_size);

    println!("\n🏆 CHAMPIONSHIP ROUNDS 🏆");
    println!("The victorious armies now face new challengers!\n");

    let champion_army1 = battle_enhanced(victorious_army1, enemy_army1);
    println!(
        "\nChampion Army 1: {} warriors remain",
        champion_army1.len()
    );

    let champion_army2 = battle_enhanced(victorious_army2, enemy_army2);
    println!(
        "\nChampion Army 2: {} warriors remain",
        champion_army2.len()
    );

    println!("\n🥇 FINAL BATTLE 🥇");
    println!("The ultimate showdown begins!\n");

    let winner = battle_enhanced(champion_army1, champion_army2);

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

pub fn start_enhanced_game(player_name: &str, army_size: i32) -> Vec<Warrior> {
    println!("\n⚔️  {}'s Battle Begins! ⚔️", player_name);
    let player_army = army(army_size);
    let enemy_army = army(army_size);

    println!("{} army count: {}", player_name, player_army.len());
    println!("Enemy army count: {}", enemy_army.len());

    let victorious_army = battle_enhanced(player_army, enemy_army);
    println!(
        "{}'s victorious army: {} warriors remain",
        player_name,
        victorious_army.len()
    );

    victorious_army
}
