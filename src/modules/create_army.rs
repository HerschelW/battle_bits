use rand::Rng;
use std::vec::Vec;

use super::constants::*;
use super::warrior::{Warrior, WarriorType};
use super::warrior_enhanced::EnhancedWarrior;

pub fn army(param: i32) -> Vec<Warrior> {
    let mut army_vector: Vec<Warrior> = Vec::new();
    let warrior_types = [WarriorType::Rock, WarriorType::Paper, WarriorType::Scissors];

    for i in 0..param {
        let warrior_type = warrior_types[rand::thread_rng().gen_range(0..3)].clone();
        let warrior = Warrior {
            name: format!(
                "{}{}",
                match warrior_type {
                    WarriorType::Rock => "Rock",
                    WarriorType::Paper => "Paper",
                    WarriorType::Scissors => "Scissors",
                },
                i + 1
            ),
            warrior_type,
            health: BASE_HEALTH,
            attack_modifier: rand::thread_rng().gen_range(MIN_STAT_MODIFIER..=MAX_STAT_MODIFIER),
            defense_modifier: rand::thread_rng().gen_range(MIN_STAT_MODIFIER..=MAX_STAT_MODIFIER),
            attack: 0,
            defense: 0,
            damage: 0,
            attack_roll: 0,
            defense_roll: 0,
            critical_hit: false,
            critical_hit_damage: 0,
            hit: false,
        };
        army_vector.push(warrior);
    }
    army_vector
}

/// Create an enhanced army with the new type balance and limitations
pub fn enhanced_army(param: i32) -> Vec<EnhancedWarrior> {
    let mut army_vector: Vec<EnhancedWarrior> = Vec::new();
    let warrior_types = [WarriorType::Rock, WarriorType::Paper, WarriorType::Scissors];

    for i in 0..param {
        let warrior_type = warrior_types[rand::thread_rng().gen_range(0..3)].clone();
        let warrior = EnhancedWarrior::new(
            format!(
                "{}{}",
                match warrior_type {
                    WarriorType::Rock => "Rock",
                    WarriorType::Paper => "Paper",
                    WarriorType::Scissors => "Scissors",
                },
                i + 1
            ),
            warrior_type,
        );
        army_vector.push(warrior);
    }
    army_vector
}
