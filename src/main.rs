use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::io::prelude::*;

mod page_reader;

pub struct Character {
	name: String,
	skill: i32,
	stamina: i32,
	luck: i32,
	provisions: i32
}

fn main() {
    println!("Hello, adventurer!");
	
	let mut player = create_character();
	
	print_character(&player);
  
  let page_data = page_reader(1);
  
	//let mut goblin = create_goblin();
	
	pause();
	
	//Battle(&mut player, &mut goblin);
}

fn print_character(x: &Character) {
	println!("Character: {}", x.name);
	println!("Skill: {}", x.skill);
	println!("Stamina: {}", x.stamina);
	println!("Luck: {}", x.luck);
	println!("Provisions: {}", x.provisions);
}

fn create_character() -> Character{
	println!("Enter your name: ");
	
	let mut name = String::new();
		
	io::stdin().read_line(&mut name)
		.expect("Failed to read line!");
		
	name = name.trim_end().to_string();
	
	let skill = roll_d6(1) + 6;
	let stamina = roll_d6(1) + 12;
	let luck = roll_d6(1) + 6;
	
	Character {
		name: name,
		skill: skill,
		stamina: stamina,
		luck: luck,
		provisions: 10
	}		
}

fn create_goblin() -> Character{
	let name = String::from("Goblin");
	
	let skill = roll_d6(1) + 6;
	let stamina = roll_d6(1) + 12;
	
	Character {
		name: name,
		skill: skill,
		stamina: stamina,
		luck: 0,
		provisions: 0
	}		
}

fn roll_d6(number_of_rolls: u32) -> i32 {
	
	let mut total_score = 0;
	let mut completed_rolls = 0;
	
	
	while completed_rolls != number_of_rolls {
		let value = rand::thread_rng().gen_range(1, 7);
		total_score += value;
		completed_rolls += 1;
	}
	
	total_score
}

fn Battle(player: &mut Character, enemy: &mut Character) {
	println!("You traverse the twists and turns of the dark, gloomy cave.");
	println!("You take a sharp turn, only to find an angry {} jump out and attack!", enemy.name);
	
	println!(" ");
	print_character(&player);
	println!(" ");
	print_character(&enemy);
	println!(" ");
		
	pause();
	
	while player.stamina > 0 && enemy.stamina > 0 {
	
		let player_attack_strength = roll_d6(2) + player.skill;
		let enemy_attack_strength = roll_d6(2) + enemy.skill;
		
		match player_attack_strength.cmp(&enemy_attack_strength) {
			Ordering::Less => {
				println!("{} hit {} for 2 damage!", enemy.name, player.name);				
				player.stamina -= 2;
			}
			
			Ordering::Greater => {
				println!("{} hit {} for 2 damage!", player.name, enemy.name);
				enemy.stamina -= 2;
			}
			
			Ordering::Equal => println!("Both {} and {} avoided each other's attacks!", player.name, enemy.name),
			}
		
		println!(" ");
		print_character(&player);
		println!(" ");
		print_character(&enemy);
		println!(" ");
		
		pause();
	}
	
	if player.stamina <= 0 {
		println!("YOU DIED! GAME OVER!");
	}
	else if enemy.stamina <= 0 {
		println!("Congratulations, you defeated the {}", enemy.name);
	}
	
	pause();
}

fn pause() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...\n").unwrap();
    stdout.flush().unwrap();

    let mut answer = String::new();

    stdin.read_line(&mut answer)
      .ok()
      .expect("Failed to read line");
}