use super::warrior::Warrior;

pub fn print_battle_header(round: usize, player_warrior: &Warrior, enemy_warrior: &Warrior) {
    println!("\n{}", "=".repeat(60));
    println!("BATTLE ROUND {}", round);
    println!("{}", "=".repeat(60));
    println!("{} vs {}", player_warrior.name, enemy_warrior.name);
    println!(
        "Type: {:?} vs {:?}",
        player_warrior.warrior_type, enemy_warrior.warrior_type
    );
    println!(
        "Health: {} vs {}",
        player_warrior.health, enemy_warrior.health
    );
    println!(
        "Stats: ATK:{}/DEF:{} vs ATK:{}/DEF:{}",
        player_warrior.attack_modifier,
        player_warrior.defense_modifier,
        enemy_warrior.attack_modifier,
        enemy_warrior.defense_modifier
    );
}

pub fn print_attack_result(attacker: &str, damage: i32, critical: bool) {
    if critical {
        println!("💥 {} CRITICAL HIT! {} damage!", attacker, damage);
    } else if damage > 0 {
        println!("⚔️  {} attacks for {} damage!", attacker, damage);
    } else {
        println!("🛡️  {}'s attack is blocked!", attacker);
    }
}

pub fn print_army_status(army_name: &str, count: usize) {
    let bar_length = 20;
    let filled = (count as f32 / 500.0 * bar_length as f32) as usize;
    let bar = "█".repeat(filled) + &"░".repeat(bar_length - filled);
    println!("{} Army: {} warriors [{}] {}", army_name, count, bar, count);
}

pub fn print_victory_message(winner: &str, remaining_warriors: usize) {
    println!("\n{}", "🎉".repeat(20));
    println!(
        "{} VICTORY! {} warriors remain!",
        winner, remaining_warriors
    );
    println!("{}", "🎉".repeat(20));
}
