pub mod modules {
    pub mod battle;
    pub mod battle_enhanced;
    pub mod combat_calculator;
    pub mod constants;
    pub mod create_army;
    pub mod visualization;
    pub mod warrior;
    pub mod warrior_enhanced;
}

use modules::create_army::enhanced_army;
use modules::warrior_enhanced::EnhancedWarrior;

fn main() {
    println!("🎮 BATTLE BITS - Enhanced Warrior Demo 🎮");
    println!("Showing the new type balance and limitations!\n");
    
    // Create a small enhanced army to demonstrate
    let army = enhanced_army(6);
    
    println!("📊 ENHANCED WARRIOR STATISTICS:");
    println!("{}", "=".repeat(60));
    
    for warrior in &army {
        let (health_mod, attack_mod, defense_mod) = warrior.warrior_type.get_enhanced_modifiers();
        
        println!("{}", "─".repeat(60));
        println!("Name: {}", warrior.name);
        println!("Type: {:?}", warrior.warrior_type);
        println!("Health: {} (Base: {}, Modifier: {:.1}x)", 
                 warrior.health, 100, health_mod);
        println!("Attack Modifier: {} (Base: {}, Modifier: {:.1}x)", 
                 warrior.attack_modifier, 50, attack_mod);
        println!("Defense Modifier: {} (Base: {}, Modifier: {:.1}x)", 
                 warrior.defense_modifier, 50, defense_mod);
        println!("Fatigue: {}/100", warrior.fatigue);
        println!("Level: {}/10", warrior.level);
        println!("Experience: {}", warrior.experience);
        println!("Combat Score: {}", warrior.get_combat_score());
    }
    
    println!("{}", "=".repeat(60));
    println!("\n🎯 TYPE BALANCE SUMMARY:");
    println!("• Rock: Tanky (+20% health, -10% attack)");
    println!("• Paper: Defensive (-10% health, +20% defense)");
    println!("• Scissors: Offensive (+20% attack, -10% defense)");
    
    println!("\n🚀 ENHANCED FEATURES:");
    println!("• Stat caps prevent infinite growth");
    println!("• Fatigue system forces army rotation");
    println!("• Experience and leveling system");
    println!("• Type-specific characteristics");
    println!("• Combat effectiveness scoring");
    
    println!("\nPress Enter to see a quick battle simulation...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    // Simulate a few battles
    println!("\n⚔️  SIMULATING BATTLES ⚔️");
    let mut army1 = enhanced_army(3);
    let mut army2 = enhanced_army(3);
    
    for i in 0..3 {
        if i < army1.len() && i < army2.len() {
            println!("\n--- Battle {} ---", i + 1);
            println!("{} vs {}", army1[i].name, army2[i].name);
            println!("Type: {:?} vs {:?}", army1[i].warrior_type, army2[i].warrior_type);
            
            // Simulate battle effects
            army1[i].increase_fatigue(15);
            army2[i].increase_fatigue(15);
            army1[i].gain_experience(25);
            army2[i].gain_experience(25);
            
            println!("After battle:");
            println!("{} - Fatigue: {}, Level: {}, XP: {}", 
                     army1[i].name, army1[i].fatigue, army1[i].level, army1[i].experience);
            println!("{} - Fatigue: {}, Level: {}, XP: {}", 
                     army2[i].name, army2[i].fatigue, army2[i].level, army2[i].experience);
        }
    }
    
    println!("\n🎉 Demo complete! The enhanced system is working!");
}
