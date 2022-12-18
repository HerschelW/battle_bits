// make battle game
// 2019-10-20
// github.com/HerschelW

// use rand::Rng;
use rand::Rng;

pub fn battle() {
    // wait for enter function
    fn wait_for_enter() {
        // wait for enter
        println!("Press enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // check if enter was pressed
        if input == "enter" {
        } else {
            println!("Press enter to continue...");
        }
    }

    // player health
    let mut _player_health = 100;
    // player attack modifier
    let mut _player_attack_modifier = 10;
    // player defense modifier
    let mut _player_defense_modifier = 10;
    // player attack
    let mut _player_attack = 0;
    // player defense
    let mut _player_defense = 0;
    // player damage
    let mut _player_damage = 0;
    // player attack roll
    let mut _player_attack_roll = 0;
    // player defense roll
    let mut _player_defense_roll = 0;
    // critial hit
    let mut _player_critical_hit = false;
    // critical hit damage
    let mut _player_critical_hit_damage = 0;
    // player hit
    let mut player_hit = false;

    // enemy health
    let mut _enemy_health = 100;
    // enemy attack modifier
    let mut _enemy_attack_modifier = 10;
    // enemy defense modifier
    let mut _enemy_defense_modifier = 10;
    // enemy attack
    let mut _enemy_attack = 0;
    // enemy defense
    let mut _enemy_defense = 0;
    // enemy damage
    let mut _enemy_damage = 0;
    // enemy attack roll
    let mut _enemy_attack_roll = 0;
    // enemy defense roll
    let mut _enemy_defense_roll = 0;
    // critial hit
    let mut _enemy_critical_hit = false;
    // critical hit damage
    let mut _enemy_critical_hit_damage = 0;
    // enemy hit
    let mut enemy_hit = false;

    // round number
    let mut round = 0;

    // loop
    loop {
        // print round number
        round += 1;
        println!("Round {}", round);
        // player attack roll
        _player_attack_roll = rand::thread_rng().gen_range(1..=20);

        // enemy attack roll
        _enemy_attack_roll = rand::thread_rng().gen_range(1..=20);

        // player defense roll
        _player_defense_roll = rand::thread_rng().gen_range(1..=20);

        // enemy defense roll
        _enemy_defense_roll = rand::thread_rng().gen_range(1..=20);

        // player critical hit
        if _player_attack_roll == 20 {
            _player_critical_hit = true;
            _player_critical_hit_damage = rand::thread_rng().gen_range(1..=20);
        }
        // enemy critical hit
        if _enemy_attack_roll == 20 {
            _enemy_critical_hit = true;
            _enemy_critical_hit_damage = rand::thread_rng().gen_range(1..=20);
        }

        // player attack
        if _player_critical_hit {
            _player_attack =
                _player_attack_roll + _player_attack_modifier + _player_critical_hit_damage;
            println!("Player critical hit!");
        } else {
            _player_attack = _player_attack_roll + _player_attack_modifier;
        }

        // enemy attack
        if _enemy_critical_hit {
            _enemy_attack =
                _enemy_attack_roll + _enemy_attack_modifier + _enemy_critical_hit_damage;
            println!("Enemy critical hit!");
        } else {
            _enemy_attack = _enemy_attack_roll + _enemy_attack_modifier;
        }

        // player defense
        _player_defense = _player_defense_roll + _player_defense_modifier;

        // enemy defense
        _enemy_defense = _enemy_defense_roll + _enemy_defense_modifier;

        // player damage
        if _enemy_defense < _player_attack {
            _player_damage = _player_attack - _enemy_defense;
            player_hit = true;
            println!("Player attack hit for {} damage!", _player_damage);
        } else {
            _player_damage = 0;
            println!("Player attack missed!");
            _enemy_health += 5;
        }

        // enemy damage
        if _player_defense < _enemy_attack {
            _enemy_damage = _enemy_attack - _player_defense;
            enemy_hit = true;
            println!("Enemy attack hit for {} damage!", _enemy_damage);
        } else {
            _enemy_damage = 0;
            println!("Enemy attack missed!");
            _player_health += 5;
        }

        // if player hit is true
        if player_hit {
            _enemy_health -= _player_damage;
            _enemy_defense_modifier -= 1;
        }

        // if enemy hit is true
        if enemy_hit {
            _player_health -= _enemy_damage;
            _player_defense_modifier -= 1;
        }

        // if player defense modifier is 0
        if _player_defense_modifier <= 0 {
            _player_defense_modifier = 0;
        }

        // if enemy defense modifier is 0
        if _enemy_defense_modifier <= 0 {
            _enemy_defense_modifier = 0;
        }

        // if player health is 0
        if _player_health <= 0 {
            _player_health = 0;
        }

        // if enemy health is 0
        if _enemy_health <= 0 {
            _enemy_health = 0;
        }

        // max player health
        if _player_health > 100 {
            _player_health = 100;
        }

        // max enemy health
        if _enemy_health > 100 {
            _enemy_health = 100;
        }

        println!("Player Health: {}", _player_health);
        println!("Enemy Health: {}", _enemy_health);
        println!("Player Attack: {}", _player_attack);
        println!("Enemy Defense: {}", _enemy_defense);
        println!("Enemy Attack: {}", _enemy_attack);
        println!("Player Defense: {}", _player_defense);
        println!("Player Damage: {}", _player_damage);
        println!("Enemy Damage: {}", _enemy_damage);
        println!("Player Attack Roll: {}", _player_attack_roll);
        println!("Enemy Defense Roll: {}", _enemy_defense_roll);
        println!("Enemy Attack Roll: {}", _enemy_attack_roll);
        println!("Player Defense Roll: {}", _player_defense_roll);

        // if player health is 0
        if _player_health <= 0 {
            println!("Player is dead!\n\n");
            break;
        }
        // if enemy health is 0
        if _enemy_health <= 0 {
            println!("Enemy is dead!\n\n");
            break;
        }

        // reset player hit
        player_hit = false;
        // reset enemy hit
        enemy_hit = false;
        // reset player critical hit
        _player_critical_hit = false;
        // reset enemy critical hit
        _enemy_critical_hit = false;

        wait_for_enter();
        // create a 3 blank lines
        println!();
        println!();
        println!();
    }
}
