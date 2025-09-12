use super::combat_calculator::CombatCalculator;
use super::visualization::*;
use super::warrior::Warrior;

pub fn battle(mut player_army: Vec<Warrior>, mut enemy_army: Vec<Warrior>) -> Vec<Warrior> {
    // fn wait_for_enter() {
    //   println!("Press enter to continue...");
    //   let mut input = String::new();
    //   std::io::stdin().read_line(&mut input).unwrap();

    //   if input.trim() == "" {
    //     println!("Continuing...");
    //   } else {
    //     println!("Invalid input!");
    //     wait_for_enter();
    //   }
    // }

    fn pause_one_second() {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    let player_army_size = player_army.len();
    let enemy_army_size = enemy_army.len();
    let mut _number_of_battles = 0;
    if player_army_size < enemy_army_size {
        _number_of_battles = player_army_size;
    } else {
        _number_of_battles = enemy_army_size;
    }

    let mut i = 0;
    // wait_for_enter();
    pause_one_second();
    loop {
        if i >= _number_of_battles {
            println!("📊 Counting casualties...");
            player_army.retain(|warrior| warrior.health > 0);
            enemy_army.retain(|warrior| warrior.health > 0);

            // Enhanced visualization with progress bars
            print_army_status("Player", player_army.len());
            print_army_status("Enemy", enemy_army.len());

            if player_army.is_empty() {
                print_victory_message("Enemy", enemy_army.len());
                return enemy_army;
            } else if enemy_army.is_empty() {
                print_victory_message("Player", player_army.len());
                return player_army;
            }

            _number_of_battles = std::cmp::min(player_army.len(), enemy_army.len());
            println!("⚔️  Next round: {} battles remaining", _number_of_battles);
            i = 0;
        }

        let mut player_health = player_army[i].health;
        let player_attack_modifier = player_army[i].attack_modifier;
        let mut player_defense_modifier = player_army[i].defense_modifier;
        let mut enemy_health = enemy_army[i].health;
        let enemy_attack_modifier = enemy_army[i].attack_modifier;
        let mut enemy_defense_modifier = enemy_army[i].defense_modifier;
        let mut _round = 0;

        // Print battle header for this warrior duel
        print_battle_header(i + 1, &player_army[i], &enemy_army[i]);

        loop {
            _round += 1;

            // Cap health at maximum
            player_health = CombatCalculator::cap_health(player_health);
            enemy_health = CombatCalculator::cap_health(enemy_health);

            // Create temporary warriors for combat calculation
            let temp_player = Warrior {
                name: player_army[i].name.clone(),
                warrior_type: player_army[i].warrior_type.clone(),
                health: player_health,
                attack_modifier: player_attack_modifier,
                defense_modifier: player_defense_modifier,
                attack: 0,
                defense: 0,
                damage: 0,
                attack_roll: 0,
                defense_roll: 0,
                critical_hit: false,
                critical_hit_damage: 0,
                hit: false,
            };

            let temp_enemy = Warrior {
                name: enemy_army[i].name.clone(),
                warrior_type: enemy_army[i].warrior_type.clone(),
                health: enemy_health,
                attack_modifier: enemy_attack_modifier,
                defense_modifier: enemy_defense_modifier,
                attack: 0,
                defense: 0,
                damage: 0,
                attack_roll: 0,
                defense_roll: 0,
                critical_hit: false,
                critical_hit_damage: 0,
                hit: false,
            };

            // Process combat for both warriors
            let player_combat = CombatCalculator::process_combat_round(&temp_player);
            let enemy_combat = CombatCalculator::process_combat_round(&temp_enemy);

            // Resolve combat between them
            let (player_result, enemy_result) =
                CombatCalculator::resolve_combat(&player_combat, &enemy_combat);

            // Display attack results
            print_attack_result(
                &player_army[i].name,
                player_result.damage,
                player_result.critical_hit,
            );
            print_attack_result(
                &enemy_army[i].name,
                enemy_result.damage,
                enemy_result.critical_hit,
            );

            // Apply damage and bonuses
            if player_result.hit {
                enemy_health -= player_result.damage;
                enemy_defense_modifier =
                    (enemy_defense_modifier - CombatCalculator::get_defense_penalty()).max(0);
            } else {
                enemy_health += CombatCalculator::get_miss_bonus();
            }

            if enemy_result.hit {
                player_health -= enemy_result.damage;
                player_defense_modifier =
                    (player_defense_modifier - CombatCalculator::get_defense_penalty()).max(0);
            } else {
                player_health += CombatCalculator::get_miss_bonus();
            }

            // Ensure health doesn't go below 0
            player_health = player_health.max(0);
            enemy_health = enemy_health.max(0);

            // Check for battle end conditions
            if player_health <= 0 && enemy_health <= 0 {
                i += 1;
                break;
            }

            if player_health <= 0 {
                println!("💀 {} is defeated!", player_army[i].name);
                player_army[i].health = player_health;
                enemy_army[i].health = enemy_health + CombatCalculator::get_victory_health_bonus();
                enemy_army[i].attack_modifier += CombatCalculator::get_victory_stat_bonus();
                enemy_army[i].defense_modifier += CombatCalculator::get_victory_stat_bonus();
                i += 1;
                break;
            } else if enemy_health <= 0 {
                println!("💀 {} is defeated!", enemy_army[i].name);
                enemy_army[i].health = enemy_health;
                player_army[i].health =
                    player_health + CombatCalculator::get_victory_health_bonus();
                player_army[i].attack_modifier += CombatCalculator::get_victory_stat_bonus();
                player_army[i].defense_modifier += CombatCalculator::get_victory_stat_bonus();
                i += 1;
                break;
            }

            println!("\n");
        }
    }
}
