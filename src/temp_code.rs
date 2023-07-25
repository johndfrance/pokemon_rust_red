use crate::read_user_input;
use std::io;
use std::io::Write;

fn old_battle_fn() {
    /*
    loop {
        println!(
            "Player {}(LVL{}) has {}/{}HP.",
            player1.name, player1.level, player1.current_hp, player1.max_hp.value
        );
        println!(
            "Enemy {}(LVL{}) has {}/{}HP.",
            player2.name, player2.level, player2.current_hp, player2.max_hp.value
        );
        println!("Player 1 Select a Move: ");
        let mut move_count = 1;
        for moves in &player1.moves{
            print!(" {}. {}", move_count, moves.move_stats().name);
            move_count +=1;
        }
        io::stdout().flush().unwrap();
        /*
        println!(
            "1.{} 2.{}",
            player1.move_name(),
            player1.second_move_name()
        );
         */
        let player1_input = read_user_input();
        let player1_input = player1_input.as_str();

        println!("player 2 Select a Move: ");
        println!(
            "1.{} 2. {}",
            player2.move_name(),
            player2.second_move_name()
        );
        let player2_input = read_user_input();
        let player2_input = player2_input.as_str();

        if player1.spd.value >= player2.spd.value {
            match player1_input {
                "1" => {
                    println!("\n{} used {}", player1.name, player1.moves[0].move_stats().name);
                    player2.damage(&player1, &player1.moves[0])
                }
                "2" => {
                    println!("\n{} used {}", player1.name, player1.moves[1].move_stats().name);
                    player2.damage(&player1, &player1.moves[1])
                }
                "3"=>{
                    println!("\n{} used {}", player1.name, player1.moves[2].move_stats().name); // Will Crash if there is less than 3 moves and 3 is selected
                    player2.damage(&player1, &player1.moves[2])
                }
                "4"=>{
                    println!("\n{} used {}",player1.name, player1.moves[3].move_stats().name);
                    player2.damage(&player1, &player1.moves[3])
                }
                _ => println!("Invalid choice"),
            }
            if player2.current_hp == 0 {
                winner = true;
                println!("\nPlayer 2 fainted!");
                break;
            } else {
                match player2_input {
                    "1" => {
                        println!("\nPlayer 2 selected {}", player2.move_name());
                        player1.damage(&player2, &player2.first_move)
                    }
                    "2" => {
                        println!("\nPlayer 2 selected {}", player2.second_move_name());
                        player1.damage(&player2, &player2.second_move)
                    }
                    _ => println!("Invalid choice"),
                }
            }
            if player1.current_hp == 0 {
                winner = false;
                println!("\nPlayer 1 fainted!");
                break;
            }
        } else {
            match player2_input {
                "1" => {
                    println!("\nPlayer 2 selected {}", player2.move_name());
                    player1.damage(&player2, &player2.first_move)
                }
                "2" => {
                    println!("\nPlayer 2 selected {}", player2.second_move_name());
                    player1.damage(&player2, &player2.second_move)
                }
                _ => println!("Invalid choice"),
            }

            if player1.current_hp == 0 {
                winner = false;
                println!("\nPlayer 1 fainted!");
                break;
            }
            match player1_input {
                "1" => {
                    println!("\nPlayer 1 selected {}", player1.move_name());
                    player2.damage(&player1, &player1.first_move)
                }
                "2" => {
                    println!("\nPlayer 1 selected {}", player1.second_move_name());
                    player2.damage(&player1, &player1.second_move)
                }
                _ => println!("Invalid choice"),
            }
            if player2.current_hp == 0 {
                winner = true;
                println!("\nPlayer 2 fainted!");
                break;
            }
        }
    }
    if winner {
        player1.gain_exp(&player2);
        player1.check_level_up();
    }

     */
}
// OLD GAME START
/*
println!("POKEMON - RUST RED");
let msg1 = "\nWhat is your name?";
type_text(msg1);

let player_name = read_user_input();
let mut player = Player {
    name: player_name.clone(),
    poke_team: vec![],
    cash: 0,
};
let msg2 = format!("Welcome to the world of Pokemon {}!\n", player_name);
let msg2 = msg2.as_str();
type_text(msg2);
println!(
    "Choose your starting Pokemon: \n\
1. Bulbasaur\n\
2. Charmander\n\
3. Squirtle\n\
4. Pikachu"
);
let mut choice = true;
let bulbasaur = Pokemon::new(Bulbasaur, 6);
let charmander = Pokemon::new(Charamander, 5);
let squirtle = Pokemon::new(Squirtle, 5);
let pikachu = Pokemon::new(Pikachu, 5);

while choice {
    let starter_choice = read_user_input();
    let starter_choice = starter_choice.as_str();
    match starter_choice {
        "1" => {
            player.poke_team.push(bulbasaur.clone());
            choice = false;
        }
        "2" => {
            player.poke_team.push(charmander.clone());
            choice = false;
        }
        "3" => {
            player.poke_team.push(squirtle.clone());
            choice = false;
        }
        "4" => {
            player.poke_team.push(pikachu.clone());
            choice = false;
        }
        _ => println!("Sorry, that wasn't a valid choice."),
    }
}
game(player);

 */

/*
struct Events{
    enemy_trainers: HashMap<u16, Bool>,
    events: HashMap<u16, Bool>
}
*/
/*
fn game(mut player: Player) {
    let mut player1 = player.poke_team[0].clone();

    let mut opponent_count = 1;
    loop {
        let enemy = generate_enemy();
        println!(
            "\nEnemy {}: {} - Level: {}",
            opponent_count, enemy.name, enemy.level
        );
        println!("*************************************");
        let still_alive = battle(&mut player1, enemy);
        if !still_alive {
            break;
        }
        opponent_count += 1;
    }
    println!("GAME OVER")
}

fn generate_enemy() -> Pokemon {
    let variants = [
        Pikachu,
        Bulbasaur,
        Charamander,
        Squirtle,
        Pidgey,
        // Add more Pokémon variants
    ];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..variants.len());
    let mut lvl_rng = rand::thread_rng();
    let enemy_lvl = lvl_rng.gen_range(1..5);
    let enemy_species = variants[index].clone();
    let enemy = Pokemon::new(enemy_species, enemy_lvl);
    return enemy;
}

 */
