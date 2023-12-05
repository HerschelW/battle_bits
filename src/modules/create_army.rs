use rand::Rng;
use std::vec::Vec;

use super::warrior::Warrior;

pub fn army(param: i32) -> Vec<Warrior> {
    let mut army_vector: Vec<Warrior> = Vec::new();

    for i in 0..param {
        let warrior = Warrior {
            name: "Warrior".to_string() + (i + 1).to_string().as_str(),
            health: 100,
            attack_modifier: rand::thread_rng().gen_range(1..=50),
            defense_modifier: rand::thread_rng().gen_range(1..=50),
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
