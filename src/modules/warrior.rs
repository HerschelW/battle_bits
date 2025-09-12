#[derive(Debug, Clone, PartialEq)]
pub enum WarriorType {
    Rock,
    Paper,
    Scissors,
}

impl WarriorType {
    pub fn beats(&self, other: &WarriorType) -> bool {
        match (self, other) {
            (WarriorType::Rock, WarriorType::Scissors) => true,
            (WarriorType::Paper, WarriorType::Rock) => true,
            (WarriorType::Scissors, WarriorType::Paper) => true,
            _ => false,
        }
    }

    pub fn get_advantage_multiplier(&self, other: &WarriorType) -> f32 {
        if self.beats(other) {
            1.5
        } else if other.beats(self) {
            0.75
        } else {
            1.0
        }
    }
}

#[derive(Debug)]
pub struct Warrior {
    pub name: String,
    pub warrior_type: WarriorType,
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
