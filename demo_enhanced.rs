// Demo script to showcase enhanced Battle Bits features
use std::io::{self, Write};

fn main() {
    println!("🎮 BATTLE BITS - Enhanced Features Demo 🎮\n");

    println!("✨ NEW FEATURES ADDED:");
    println!("1. 🪨 Rock, Paper, Scissors warrior types with type advantages");
    println!("2. 📊 Visual battle progress with army status bars");
    println!("3. ⚔️  Enhanced combat with type multipliers");
    println!("4. 🎯 Constants for easy configuration");
    println!("5. 🎨 Better battle visualization with emojis");
    println!("6. 🏆 Improved victory messages and statistics\n");

    println!("Press Enter to run a quick demo battle...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("\n🚀 Starting Enhanced Battle Demo...\n");

    // Run the enhanced version
    std::process::Command::new("cargo")
        .args(&["run", "--bin", "bit_battle"])
        .status()
        .expect("Failed to run enhanced battle");
}
