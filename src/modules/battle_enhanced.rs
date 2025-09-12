use super::constants::*;
use super::visualization::*;
use super::warrior::{Warrior, WarriorType};
use rand::Rng;

pub fn battle_enhanced(
    mut player_army: Vec<Warrior>,
    mut enemy_army: Vec<Warrior>,
) -> Vec<Warrior> {
    let mut round = 0;

    loop {
        // Remove dead warriors
        player_army.retain(|warrior| warrior.health > 0);
        enemy_army.retain(|warrior| warrior.health > 0);

        print_army_status("Player", player_army.len());
        print_army_status("Enemy", enemy_army.len());

        if player_army.is_empty() {
            print_victory_message("Enemy", enemy_army.len());
            return enemy_army;
        } else if enemy_army.is_empty() {
            print_victory_message("Player", player_army.len());
            return player_army;
        }

        // Battle the first warriors from each army
        let battle_result = single_warrior_battle(&mut player_army[0], &mut enemy_army[0], round);

        match battle_result {
            BattleResult::PlayerWins => {
                // Player warrior gets bonuses
                player_army[0].health += HEALTH_BONUS_ON_VICTORY;
                player_army[0].attack_modifier += STAT_BONUS_ON_VICTORY;
                player_army[0].defense_modifier += STAT_BONUS_ON_VICTORY;
                enemy_army.remove(0);
            }
            BattleResult::EnemyWins => {
                // Enemy warrior gets bonuses
                enemy_army[0].health += HEALTH_BONUS_ON_VICTORY;
                enemy_army[0].attack_modifier += STAT_BONUS_ON_VICTORY;
                enemy_army[0].defense_modifier += STAT_BONUS_ON_VICTORY;
                player_army.remove(0);
            }
            BattleResult::BothDie => {
                // Both warriors die
                player_army.remove(0);
                enemy_army.remove(0);
            }
        }

        round += 1;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[derive(Debug)]
enum BattleResult {
    PlayerWins,
    EnemyWins,
    BothDie,
}

fn single_warrior_battle(player: &mut Warrior, enemy: &mut Warrior, round: usize) -> BattleResult {
    print_battle_header(round, player, enemy);

    let mut player_health = player.health;
    let mut enemy_health = enemy.health;
    let mut battle_round = 0;

    loop {
        battle_round += 1;

        // Cap health at maximum
        if player_health > MAX_HEALTH {
            player_health = MAX_HEALTH;
        }
        if enemy_health > MAX_HEALTH {
            enemy_health = MAX_HEALTH;
        }

        // Roll dice
        let player_attack_roll = rand::thread_rng().gen_range(1..=DICE_MAX);
        let enemy_attack_roll = rand::thread_rng().gen_range(1..=DICE_MAX);
        let player_defense_roll = rand::thread_rng().gen_range(1..=DICE_MAX);
        let enemy_defense_roll = rand::thread_rng().gen_range(1..=DICE_MAX);

        // Check for critical hits
        let player_critical = player_attack_roll == CRITICAL_HIT_ROLL;
        let enemy_critical = enemy_attack_roll == CRITICAL_HIT_ROLL;

        // Calculate attacks with type advantage
        let type_multiplier = player
            .warrior_type
            .get_advantage_multiplier(&enemy.warrior_type);
        let player_attack = if player_critical {
            (player_attack_roll
                + player.attack_modifier
                + rand::thread_rng().gen_range(1..=DICE_MAX)) as f32
                * type_multiplier
        } else {
            (player_attack_roll + player.attack_modifier) as f32 * type_multiplier
        } as i32;

        let enemy_attack = if enemy_critical {
            (enemy_attack_roll + enemy.attack_modifier + rand::thread_rng().gen_range(1..=DICE_MAX))
                as f32
                / type_multiplier
        } else {
            (enemy_attack_roll + enemy.attack_modifier) as f32 / type_multiplier
        } as i32;

        let player_defense = player_defense_roll + player.defense_modifier;
        let enemy_defense = enemy_defense_roll + enemy.defense_modifier;

        // Calculate damage
        let player_damage = if enemy_defense < player_attack {
            let damage = player_attack - enemy_defense;
            print_attack_result(&player.name, damage, player_critical);
            damage
        } else {
            print_attack_result(&player.name, 0, false);
            enemy_health += HEALTH_BONUS_ON_MISS;
            0
        };

        let enemy_damage = if player_defense < enemy_attack {
            let damage = enemy_attack - player_defense;
            print_attack_result(&enemy.name, damage, enemy_critical);
            damage
        } else {
            print_attack_result(&enemy.name, 0, false);
            player_health += HEALTH_BONUS_ON_MISS;
            0
        };

        // Apply damage
        if player_damage > 0 {
            enemy_health -= player_damage;
            enemy.defense_modifier = (enemy.defense_modifier - DEFENSE_PENALTY_ON_HIT).max(0);
        }

        if enemy_damage > 0 {
            player_health -= enemy_damage;
            player.defense_modifier = (player.defense_modifier - DEFENSE_PENALTY_ON_HIT).max(0);
        }

        // Ensure health doesn't go below 0
        player_health = player_health.max(0);
        enemy_health = enemy_health.max(0);

        // Check for battle end
        if player_health <= 0 && enemy_health <= 0 {
            println!("💀 Both warriors perish!");
            return BattleResult::BothDie;
        } else if player_health <= 0 {
            println!("💀 {} is defeated!", player.name);
            player.health = player_health;
            enemy.health = enemy_health;
            return BattleResult::EnemyWins;
        } else if enemy_health <= 0 {
            println!("💀 {} is defeated!", enemy.name);
            player.health = player_health;
            enemy.health = enemy_health;
            return BattleResult::PlayerWins;
        }

        // Update warrior health
        player.health = player_health;
        enemy.health = enemy_health;
    }
}
