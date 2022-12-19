pub fn warrior_profile(params: Vec<i32>) {
    pub struct Warrior {
        // name
        pub name: String,
        // health
        pub health: i32,
        // attack modifier
        pub attack_modifier: i32,
        // defense modifier
        pub defense_modifier: i32,
        // attack
        pub attack: i32,
        // defense
        pub defense: i32,
        // damage
        pub damage: i32,
        // attack roll
        pub attack_roll: i32,
        // defense roll
        pub defense_roll: i32,
        // critial hit
        pub critical_hit: bool,
        // critical hit damage
        pub critical_hit_damage: i32,
        // hit
        pub hit: bool,
    }

    // create warrior
    let mut warrior = Warrior {
        name: "Warrior".to_string(),
        health: params[0],
        attack_modifier: params[1],
        defense_modifier: params[2],
        attack: 0,
        defense: 0,
        damage: 0,
        attack_roll: 0,
        defense_roll: 0,
        critical_hit: false,
        critical_hit_damage: 0,
        hit: false,
    };
}

// create army
pub fn create_army() {
    // create empty array of warriors
    let mut warriors = Vec::new();

    // create figher
    warrior_profile(vec![100, 10, 10, 1]);

    // create mage
    warrior_profile(vec![100, 15, 5, 2]);

    // create rogue
    warrior_profile(vec![100, 5, 15, 3]);

    println!("Army: {:?}", warriors);
}
