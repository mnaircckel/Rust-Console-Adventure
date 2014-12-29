mod game;

fn main() {
	let mut actionHandler = game::new_game();
	loop{
		game::start_menu(&mut actionHandler);
		if !actionHandler.running{break;}
		loop{
			game::main_menu(&mut actionHandler);
			if actionHandler.level == -1{break;}
		}
	}
}
