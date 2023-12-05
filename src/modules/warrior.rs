#[derive(Debug)]
pub struct Warrior {
    pub name: String,
    pub health: i32,
    pub attack_modifier: i32,
    pub defense_modifier: i32,
    pub attack: i32,
    pub defense: i32,
    pub damage: i32,
    pub attack_roll: i32,
    pub defense_roll: i32,
    pub critical_hit: bool,
    pub critical_hit_damage: i32,
    pub hit: bool,
}
