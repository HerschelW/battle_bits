use super::warrior::Warrior;
use super::constants::*;
use rand::Rng;

/// Combat calculation results
#[derive(Debug)]
pub struct CombatResult {
    pub attack: i32,
    pub defense: i32,
    pub damage: i32,
    pub critical_hit: bool,
    pub hit: bool,
}

/// Handles all combat calculations and dice rolling
pub struct CombatCalculator;

impl CombatCalculator {
    /// Roll dice for attack or defense
    pub fn roll_dice() -> i32 {
        rand::thread_rng().gen_range(1..=DICE_MAX)
    }

    /// Check if a roll is a critical hit
    pub fn is_critical_hit(roll: i32) -> bool {
        roll == CRITICAL_HIT_ROLL
    }

    /// Calculate attack value including critical hit bonus
    pub fn calculate_attack(attack_roll: i32, attack_modifier: i32, critical_hit: bool) -> i32 {
        if critical_hit {
            attack_roll + attack_modifier + Self::roll_dice()
        } else {
            attack_roll + attack_modifier
        }
    }

    /// Calculate defense value
    pub fn calculate_defense(defense_roll: i32, defense_modifier: i32) -> i32 {
        defense_roll + defense_modifier
    }

    /// Calculate damage dealt (attack - defense, minimum 0)
    pub fn calculate_damage(attack: i32, defense: i32) -> i32 {
        if attack > defense {
            attack - defense
        } else {
            0
        }
    }

    /// Check if attack hits (attack > defense)
    pub fn does_attack_hit(attack: i32, defense: i32) -> bool {
        attack > defense
    }

    /// Cap health at maximum value
    pub fn cap_health(health: i32) -> i32 {
        health.min(MAX_HEALTH)
    }

    /// Process a complete combat round for a warrior
    pub fn process_combat_round(warrior: &Warrior) -> CombatResult {
        let attack_roll = Self::roll_dice();
        let defense_roll = Self::roll_dice();
        let critical_hit = Self::is_critical_hit(attack_roll);
        
        let attack = Self::calculate_attack(attack_roll, warrior.attack_modifier, critical_hit);
        let defense = Self::calculate_defense(defense_roll, warrior.defense_modifier);
        
        CombatResult {
            attack,
            defense,
            damage: 0, // Will be calculated when comparing with opponent
            critical_hit,
            hit: false, // Will be determined when comparing with opponent
        }
    }

    /// Resolve combat between two warriors
    pub fn resolve_combat(player: &CombatResult, enemy: &CombatResult) -> (CombatResult, CombatResult) {
        let mut player_result = CombatResult {
            attack: player.attack,
            defense: player.defense,
            damage: Self::calculate_damage(player.attack, enemy.defense),
            critical_hit: player.critical_hit,
            hit: Self::does_attack_hit(player.attack, enemy.defense),
        };

        let mut enemy_result = CombatResult {
            attack: enemy.attack,
            defense: enemy.defense,
            damage: Self::calculate_damage(enemy.attack, player.defense),
            critical_hit: enemy.critical_hit,
            hit: Self::does_attack_hit(enemy.attack, player.defense),
        };

        // If attack doesn't hit, no damage
        if !player_result.hit {
            player_result.damage = 0;
        }
        if !enemy_result.hit {
            enemy_result.damage = 0;
        }

        (player_result, enemy_result)
    }

    /// Apply health bonus when attack misses
    pub fn get_miss_bonus() -> i32 {
        HEALTH_BONUS_ON_MISS
    }

    /// Apply victory bonuses
    pub fn get_victory_health_bonus() -> i32 {
        HEALTH_BONUS_ON_VICTORY
    }

    /// Apply victory stat bonuses
    pub fn get_victory_stat_bonus() -> i32 {
        STAT_BONUS_ON_VICTORY
    }

    /// Apply defense penalty when hit
    pub fn get_defense_penalty() -> i32 {
        DEFENSE_PENALTY_ON_HIT
    }
}
