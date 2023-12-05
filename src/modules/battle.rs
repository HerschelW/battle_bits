use super::warrior::Warrior;
use rand::Rng;

pub fn battle(mut player_army: Vec<Warrior>, mut enemy_army: Vec<Warrior>) -> Vec<Warrior> {
  fn wait_for_enter() {
    println!("Press enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "" {
      println!("Continuing...");
    } else {
      println!("Invalid input!");
      wait_for_enter();
    }
  }

  // get starting size of each army
  let player_army_size = player_army.len();
  let enemy_army_size = enemy_army.len();
  let mut _number_of_battles = 0;
  // determine which army is smaller
  if player_army_size < enemy_army_size {
    _number_of_battles = player_army_size;
  } else {
    _number_of_battles = enemy_army_size;
  }

  let mut i = 0;

  // Battles
  loop {
    if i >= _number_of_battles {
      println!("Counting casualties...");
      // remove dead warriors from each army
      player_army.retain(|warrior| warrior.health > 0);
      enemy_army.retain(|warrior| warrior.health > 0);

      println!("Player army count: {}
            ", player_army.len());
      println!("Enemy army count: {}
            ", enemy_army.len());

      // check if either army is empt
      if player_army.is_empty() {
        println!("Player army is defeated!");
        return enemy_army;
      } else if enemy_army.is_empty() {
        println!("Enemy army is defeated!");
        return player_army;
      }

      // change number of battles to the number of remaining warriors in the smaller army
      _number_of_battles = std::cmp::min(player_army.len(), enemy_army.len());
      println!("Number of battles: {}
            ", _number_of_battles);
      i = 0;
    }

    println!("Battle {} of {}", i + 1, _number_of_battles);
    wait_for_enter();

    let mut _player_health = player_army[i].health;
    let mut _player_attack_modifier = player_army[i].attack_modifier;
    let mut _player_defense_modifier = player_army[i].defense_modifier;
    let mut _player_attack = 0;
    let mut _player_defense = 0;
    let mut _player_damage = 0;
    let mut _player_attack_roll = 0;
    let mut _player_defense_roll = 0;
    let mut _player_critical_hit = false;
    let mut _player_critical_hit_damage = 0;
    let mut player_hit = false;
    let mut _enemy_health = enemy_army[i].health;
    let mut _enemy_attack_modifier = enemy_army[i].attack_modifier;
    let mut _enemy_defense_modifier = enemy_army[i].defense_modifier;
    let mut _enemy_attack = 0;
    let mut _enemy_defense = 0;
    let mut _enemy_damage = 0;
    let mut _enemy_attack_roll = 0;
    let mut _enemy_defense_roll = 0;
    let mut _enemy_critical_hit = false;
    let mut _enemy_critical_hit_damage = 0;
    let mut enemy_hit = false;
    let mut round = 0;
    // Rounds
    loop {
      round += 1;
      println!("Round {}
            ", round);
      _player_attack_roll = rand::thread_rng().gen_range(1..=50);
      _enemy_attack_roll = rand::thread_rng().gen_range(1..=50);
      _player_defense_roll = rand::thread_rng().gen_range(1..=50);
      _enemy_defense_roll = rand::thread_rng().gen_range(1..=50);
      if _player_attack_roll == 50 {
        _player_critical_hit = true;
        _player_critical_hit_damage = rand::thread_rng().gen_range(1..=50);
      }

      if _enemy_attack_roll == 50 {
        _enemy_critical_hit = true;
        _enemy_critical_hit_damage = rand::thread_rng().gen_range(1..=50);
      }

      if _player_critical_hit {
        _player_attack = _player_attack_roll + _player_attack_modifier + _player_critical_hit_damage;
        println!("{}
                 CRITICAL HIT!", player_army[i].name);
      } else {
        _player_attack = _player_attack_roll + _player_attack_modifier;
      }

      if _enemy_critical_hit {
        _enemy_attack = _enemy_attack_roll + _enemy_attack_modifier + _enemy_critical_hit_damage;
        println!("{}
                 CRITICAL HIT!", enemy_army[i].name);
      } else {
        _enemy_attack = _enemy_attack_roll + _enemy_attack_modifier;
      }

      _player_defense = _player_defense_roll + _player_defense_modifier;
      _enemy_defense = _enemy_defense_roll + _enemy_defense_modifier;
      if _enemy_defense < _player_attack {
        _player_damage = _player_attack - _enemy_defense;
        player_hit = true;
        println!(
          "{}
                     attack hit for {}
                     damage!",
          player_army[i].name,
          _player_damage
        );
      } else {
        _player_damage = 0;
        println!("Player attack missed!");
        _enemy_health += 1;
      }

      if _player_defense < _enemy_attack {
        _enemy_damage = _enemy_attack - _player_defense;
        enemy_hit = true;
        println!(
          "{}
                     attack hit for {}
                     damage!",
          enemy_army[i].name,
          _enemy_damage
        );
      } else {
        _enemy_damage = 0;
        println!("Enemy attack missed!");
        _player_health += 1;
      }

      if player_hit {
        _enemy_health -= _player_damage;
        _enemy_defense_modifier -= 1;
      }

      if enemy_hit {
        _player_health -= _enemy_damage;
        _player_defense_modifier -= 1;
      }

      if _player_defense_modifier <= 0 {
        _player_defense_modifier = 0;
      }

      if _enemy_defense_modifier <= 0 {
        _enemy_defense_modifier = 0;
      }

      if _player_health <= 0 {
        _player_health = 0;
      }

      if _enemy_health <= 0 {
        _enemy_health = 0;
      }

      println!("{}
             Health: {}
            ", player_army[i].name, _player_health);
      println!("{}
             Health: {}
            ", enemy_army[i].name, _enemy_health);
      if _player_health <= 0 && _enemy_health <= 0 {
        println!("Both players are dead!\n");
        i += 1;
        break;
      }

      if _player_health <= 0 {
        println!("Player is dead!\n");
        player_army[i].health = _player_health;
        enemy_army[i].health = _enemy_health + 5;
        enemy_army[i].attack_modifier += 1;
        enemy_army[i].defense_modifier += 1;
        i += 1;
        break;
      } else if _enemy_health <= 0 {
        println!("Enemy is dead!\n");
        enemy_army[i].health = _enemy_health;
        player_army[i].health = _player_health + 5;
        player_army[i].attack_modifier += 1;
        player_army[i].defense_modifier += 1;
        i += 1;
        break;
      }

      player_hit = false;
      enemy_hit = false;
      _player_critical_hit = false;
      _enemy_critical_hit = false;
      println!("\n");
    }
  }
}
