#[derive(Debug)]
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
