// use std::string::String;
use std::vec::Vec;

// import Warrior structure from warrior.rs
use crate::warrior::Warrior;

// #[derive(Debug)]
// pub struct Warrior {
//     // name
//     pub name: String,
//     // health
//     pub health: i32,
//     // attack modifier
//     pub attack_modifier: i32,
//     // defense modifier
//     pub defense_modifier: i32,
//     // attack
//     pub attack: i32,
//     // defense
//     pub defense: i32,
//     // damage
//     pub damage: i32,
//     // attack roll
//     pub attack_roll: i32,
//     // defense roll
//     pub defense_roll: i32,
//     // critial hit
//     pub critical_hit: bool,
//     // critical hit damage
//     pub critical_hit_damage: i32,
//     // hit
//     pub hit: bool,
// }

// pub struct Army {
//     pub warriors: Vec<Warrior>,
// }

// pub
// pub mod test {
//     let mut army_vector: Vec<Warrior> = Vec::new();
// }

pub fn army(param: i32) -> Vec<Warrior> {
    println!("Creating army...");

    // create empty army
    let mut army_vector: Vec<Warrior> = Vec::new();

    // create 10 warriors
    for i in 0..param {
        println!("Creating warrior {}", (i + 1));
        let warrior = Warrior {
            name: "Warrior".to_string() + (i + 1).to_string().as_str(),
            health: 100,
            attack_modifier: 10,
            defense_modifier: 10,
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
    // return army
    army_vector
}
