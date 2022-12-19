// define path
use warrior::Warrior;

pub struct Army {
    pub warriors: Vec<Warrior>,
}

impl Army {
    // create army
    pub fn create_army(param: i32) {
        // create empty array of objects
        let mut warriors: Vec<Warrior> = Vec::new();

        pub fn warrior_profile(params: Vec<i32>, warriors: &mut Vec<Warrior>) {
            // create warrior
            let warrior = Warrior {
                name: "Warrior".to_string() + &params[3].to_string(),
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

            warriors.push(warrior);
        }

        // create 10 warriors
        for i in 0..param {
            // create warrior
            warrior_profile(vec![100, 10, 10, i], &mut warriors);
        }

        warriors;
    }
}
