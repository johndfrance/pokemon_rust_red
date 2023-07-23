use std::io;
use std::io::Write;
use crate::read_user_input;

fn old_battle_fn(){
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