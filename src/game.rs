use std::io;
use std::rand;
use std::io::BufferedReader;
use std::io::File;

// Game State: Handles everything in the game
pub struct GameState{
	pub running: bool,
	pub player: Character,
	pub enemy: Character,
	pub level: int,
	pub monsters: Vec<Monster>,
}

impl GameState {
	pub fn new(running: bool, player: Character, enemy: Character, level: int, monsters: Vec<Monster>) -> GameState {
		GameState {
			running: running,
			player: player,
			enemy: enemy,
			level: level,
			monsters: monsters,

		}
	}
}

// Struct for storing monster data

struct Monster {
	name: String,
	level: int,
	health: int,
}

// Character: Handles actions of individual characters
struct Character {
	name: String,
	level : int,
	max_health: int,
	current_health: int,
	alive: bool,
}

impl Character {
	pub fn new(name: String, level: int, max_health: int, current_health: int, alive: bool) -> Character {
		Character {
			name: name,
			level: level,
			max_health: max_health,
			current_health: current_health,
			alive: alive,
		}
	}

	fn print_stats(&self){
		println!("\n{}, level {}, has {} health.",self.name,self.level,self.current_health);
	}

	fn attack(&self) -> int{
		self.level * ( (rand::random::<f64>() * 4f64) as int)
	}

	fn defend(&mut self, damage: int){
		self.current_health -= damage as int;
		if self.current_health <= 0{
			self.current_health = 0;
			self.alive = false;
		}
	}

	fn rest(&mut self){
		println!("\nYou spend the night resting... ");
		println!("You feel well rested!");
		self.current_health = self.max_health;
	}

}


// Functions to run game
fn new_character() -> Character {
	println!("\nEnter a name for your character: ");
	let name = io::stdin().read_line().ok().expect("Failed to read name.");
	Character::new(name.as_slice().trim().to_string(),1,100,100,true)
}

fn new_enemy(actionHandler: &mut GameState) -> Character {
	let num = rand::random::<uint>() % 13;
	let name = actionHandler.monsters[num].name.clone();
	let level = actionHandler.monsters[num].level;
	let health = actionHandler.monsters[num].health;
	Character::new(name,level,health,health,true)
}

pub fn new_game() -> GameState {
	GameState::new(true,Character::new("".to_string(),0,0,0,false),Character::new("".to_string(),0,0,0,false),-1,load_monsters()) // return the new gameState
}

pub fn start_menu(actionHandler: &mut GameState){
	loop{
		println!("  
1. New Game
2. Load Game
3. Save Game
4. Quit
				");
		println!("Please enter a option: ");
		let input = io::stdin().read_line().ok().expect("Failed to read option.");
		match input.trim().as_slice() {
			"1" => { 
					actionHandler.player = new_character();
					actionHandler.level = 0;
					break;
					},
			"2" => { println!("Option has not been implemented."); },
			"3" => { println!("Option has not been implemented."); },
			"4" => { 
					actionHandler.running = false; 
					break;
				   },
			_ =>   { println!("Invalid option."); },
		}
	}
}

pub fn main_menu(actionHandler: &mut GameState){
	println!("  
1. Main Menu
2. Battle
3. Display Statistics
4. Rest
			");
	println!("Please enter a option: ");
	let input = io::stdin().read_line().ok().expect("Failed to read option.");
	match input.trim().as_slice() {
		"1" => { actionHandler.level = -1;},
		"2" => { battle_menu(actionHandler);},
		"3" => { actionHandler.player.print_stats();},
		"4" => { actionHandler.player.rest();},
		_ =>   { println!("Invalid option."); },
	}

}

fn battle_menu(actionHandler: &mut GameState){
	actionHandler.enemy = new_enemy(actionHandler);
	actionHandler.enemy.print_stats();
	actionHandler.player.print_stats();
	while actionHandler.enemy.alive && actionHandler.player.alive {

		println!("  
1. Attack
2. Flee
				");
		println!("Please enter a option: ");
		let input = io::stdin().read_line().ok().expect("Failed to read option.");
		match input.trim().as_slice() {
			"1" => {
					actionHandler.enemy.defend(actionHandler.player.attack());
					actionHandler.player.defend(actionHandler.enemy.attack());
					actionHandler.enemy.print_stats();
					actionHandler.player.print_stats();
				},
			"2" => { 
				return;
				},
			_ =>   { println!("Invalid option."); },
		}
	}
	if actionHandler.player.alive{
		println!("You won the battle!");
	}
	else{
		println!("Oh no! You are dead!");
	}
}

//Load monster database
fn load_monsters() -> Vec<Monster> {

	let mut monsters: Vec<Monster> = vec![];
	monsters.push(Monster{
	name: "Goblin".to_string(), level: 3, health: 60
	});	
	monsters.push(Monster{
	name: "Rat".to_string(), level: 1, health: 10
	});
	monsters.push(Monster{
	name: "Ogre".to_string(), level: 8, health: 180
	});
	monsters.push(Monster{
	name: "Ent".to_string(), level: 15, health: 400
	});
	monsters.push(Monster{
	name: "Green Slime".to_string(), level: 5, health: 50
	});
	monsters.push(Monster{
	name: "Skeleton".to_string(), level: 4, health: 40
	});
	monsters.push(Monster{
	name: "Black Dragon".to_string(), level: 25, health: 1200
	});
	monsters.push(Monster{
	name: "Boar".to_string(), level: 2, health: 38
	});
	monsters.push(Monster{
	name: "Theif".to_string(), level: 1, health: 50
	});
	monsters.push(Monster{
	name: "Giant Spider".to_string(), level: 3, health: 62
	});
	monsters.push(Monster{
	name: "Imp".to_string(), level: 2, health: 25
	});
	monsters.push(Monster{
	name: "Wolf".to_string(), level: 4, health: 70
	});
	monsters.push(Monster{
	name: "Witch".to_string(), level: 5, health: 110
	});
	monsters

}