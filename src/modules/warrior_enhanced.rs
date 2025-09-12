use super::constants::*;
use super::warrior::WarriorType;

/// Enhanced warrior with limitations and strategic depth
#[derive(Debug, Clone)]
pub struct EnhancedWarrior {
    pub name: String,
    pub warrior_type: WarriorType,
    pub health: i32,
    pub max_health: i32,
    pub attack_modifier: i32,
    pub defense_modifier: i32,
    pub fatigue: i32,
    pub experience: i32,
    pub level: i32,
    pub battles_fought: i32,
    // Combat state (reset each battle)
    pub attack: i32,
    pub defense: i32,
    pub damage: i32,
    pub attack_roll: i32,
    pub defense_roll: i32,
    pub critical_hit: bool,
    pub critical_hit_damage: i32,
    pub hit: bool,
}

impl EnhancedWarrior {
    /// Create a new enhanced warrior with type-specific characteristics
    pub fn new(name: String, warrior_type: WarriorType) -> Self {
        let (health_mod, attack_mod, defense_mod) = warrior_type.get_enhanced_modifiers();
        
        Self {
            name,
            warrior_type,
            health: (BASE_HEALTH as f32 * health_mod) as i32,
            max_health: (BASE_HEALTH as f32 * health_mod) as i32,
            attack_modifier: (MIN_STAT_MODIFIER as f32 * attack_mod) as i32,
            defense_modifier: (MIN_STAT_MODIFIER as f32 * defense_mod) as i32,
            fatigue: 0,
            experience: 0,
            level: 1,
            battles_fought: 0,
            attack: 0,
            defense: 0,
            damage: 0,
            attack_roll: 0,
            defense_roll: 0,
            critical_hit: false,
            critical_hit_damage: 0,
            hit: false,
        }
    }

    /// Apply fatigue penalties to stats
    pub fn apply_fatigue_penalties(&mut self) {
        if self.fatigue > 50 {
            let penalty = (self.fatigue - 50) as f32 / 100.0;
            self.attack_modifier = (self.attack_modifier as f32 * (1.0 - penalty)) as i32;
            self.defense_modifier = (self.defense_modifier as f32 * (1.0 - penalty)) as i32;
        }
    }

    /// Gain experience and potentially level up
    pub fn gain_experience(&mut self, exp: i32) {
        self.experience += exp;
        let new_level = (self.experience / 100) + 1;
        
        if new_level > self.level && new_level <= 10 {
            self.level = new_level;
            // Level up bonuses (diminishing returns)
            let bonus = 5 - (self.level / 3);
            self.max_health += bonus;
            self.health = self.max_health; // Full heal on level up
            self.attack_modifier += bonus / 2;
            self.defense_modifier += bonus / 2;
        }
    }

    /// Apply stat decay to prevent infinite growth
    pub fn apply_stat_decay(&mut self) {
        self.attack_modifier = ((self.attack_modifier as f32 * 0.98) as i32).max(MIN_STAT_MODIFIER);
        self.defense_modifier = ((self.defense_modifier as f32 * 0.98) as i32).max(MIN_STAT_MODIFIER);
        
        // Cap stats
        self.attack_modifier = self.attack_modifier.min(MAX_STAT_MODIFIER * 2);
        self.defense_modifier = self.defense_modifier.min(MAX_STAT_MODIFIER * 2);
    }

    /// Increase fatigue after battle
    pub fn increase_fatigue(&mut self, amount: i32) {
        self.fatigue = (self.fatigue + amount).min(100);
    }

    /// Recover some fatigue during rest
    pub fn recover_fatigue(&mut self) {
        self.fatigue = (self.fatigue - 10).max(0);
    }

    /// Check if warrior is too fatigued to fight effectively
    pub fn is_exhausted(&self) -> bool {
        self.fatigue >= 90
    }

    /// Get effective health considering fatigue
    pub fn get_effective_health(&self) -> i32 {
        if self.fatigue > 70 {
            (self.health as f32 * 0.8) as i32
        } else {
            self.health
        }
    }

    /// Get combat effectiveness score
    pub fn get_combat_score(&self) -> i32 {
        let health_score = self.get_effective_health();
        let attack_score = (self.attack_modifier as f32 * (1.0 - self.fatigue as f32 / 200.0)) as i32;
        let defense_score = (self.defense_modifier as f32 * (1.0 - self.fatigue as f32 / 200.0)) as i32;
        
        health_score + attack_score + defense_score
    }
}

impl WarriorType {
    /// Get enhanced modifiers for each warrior type
    pub fn get_enhanced_modifiers(&self) -> (f32, f32, f32) {
        match self {
            WarriorType::Rock => (1.2, 0.9, 1.0),    // Tanky: +20% health, -10% attack
            WarriorType::Paper => (0.9, 1.0, 1.2),   // Defensive: -10% health, +20% defense
            WarriorType::Scissors => (1.0, 1.2, 0.9), // Offensive: +20% attack, -10% defense
        }
    }

    /// Get type-specific fatigue resistance
    pub fn get_fatigue_resistance(&self) -> f32 {
        match self {
            WarriorType::Rock => 0.8,     // More resistant to fatigue
            WarriorType::Paper => 1.2,    // More susceptible to fatigue
            WarriorType::Scissors => 1.0, // Average
        }
    }

    /// Get type-specific experience gain multiplier
    pub fn get_experience_multiplier(&self) -> f32 {
        match self {
            WarriorType::Rock => 0.9,     // Slower to learn
            WarriorType::Paper => 1.1,    // Quick to learn
            WarriorType::Scissors => 1.0, // Average
        }
    }
}
