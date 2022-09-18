use std::{error::Error, collections::HashSet};

use self::config::Config;

mod cli_utils;
pub mod config;
mod field;

/// run the program
pub fn run(config: &mut Config) -> Result<(), Box<dyn Error>> {
    //DATA
    let column_letter_range = (0u8..config.field.get_dimensions()).fold(String::new(), |mut acc, i| {acc.push((i+97) as char); acc});
    let row_number_range = 0..(config.field.get_dimensions() as usize);
    //for every round
    loop {
        let mut game_over:bool = true;
        //DATA
        let mines:Vec<(u8,u8)> = (*config).field.get_mines().clone();
        let command;

        // print board state
        cli_utils::print_game_state(&config.field);
        
        // allow user to add a flag, or check the state of a square
        //input loop
        command = loop { match cli_utils::get_string_from_user_input("Enter command: ") {
            Ok(s) => { //Verify input
                //check prefix
                let prefix;
                match s.to_ascii_lowercase().chars().nth(0) {
                    Some('c') => prefix = 'c',
                    Some('f') => prefix = 'f',
                    _ => {eprintln!("invalid command prefix");continue;},
                }
                
                //check column letter
                let column_letter;
                match s.to_ascii_lowercase().chars().nth(1) {
                    Some(c) => {
                        match column_letter_range.find(c) {
                            Some(index) => column_letter = index,
                            None => {eprintln!("invalid column letter");continue;}
                        }
                    },
                    _ => {eprintln!("invalid column letter");continue;},
                }

                //check row number
                let row_number;
                match s[2..].parse::<usize>() {
                    Ok(row) => {
                        if row_number_range.contains(&row) {
                            row_number = row;
                        }
                        else { 
                            eprintln!("invalid row number");continue;
                        }
                    },
                    _ => {eprintln!("invalid row number");continue;},
                }

                break (prefix,column_letter,row_number);
            },
            Err(e) => eprintln!("{}",e),
        }};

        // clear screen
        cli_utils::reset_screen();

        // handle command
        if let Some(square) = config.field.get_square_at_mut(command.1 as isize, command.2 as isize) {
            //what prefix is being used?
            if command.0 == 'f' { //toggle flag
                if field::State::FLAGGED.eq(square.get_state()) {
                    square.set_state(field::State::HIDDEN);
                } 
                else {
                    square.set_state(field::State::FLAGGED);
                }
            }
            else if command.0 == 'c' {
                //if square is not hidden, just skip this command input
                if field::State::HIDDEN.ne(square.get_state()) {
                    continue;
                }

                //make it visible
                square.set_state(field::State::VISIBLE);

                //end game if checked square is a mine 
                if square.is_mine() { //game over
                    println!("you hit a mine, you lose");
                    // go through every mine and make it visible
                    config.field.show_mines();
                    //print updated board
                    cli_utils::print_board(&config.field);
                    break;
                }
                // if bordering a mine, continue
                if square.get_danger() > 0 {
                    continue;
                }
                // make all surrounding non-mines that aren't bordering mines visible aswell
                else {
                    //data
                    let mut backlog:HashSet<(isize,isize)> = [(square.get_position().0 as isize, square.get_position().1 as isize)].iter().cloned().collect();
                    let mut curr_queue:HashSet<(isize,isize)>;
                    loop {
                        curr_queue = backlog;
                        backlog = HashSet::new();

                        //make all squares bordering each square in curr_queue visible, and add all that have 0 danger to backlog
                        for (x_pos,y_pos) in curr_queue.iter() {
                            cli_utils::reset_screen();
                            backlog.extend(config.field.check_and_update_states_of_adjacent_squares(*x_pos, *y_pos).iter());
                            cli_utils::print_game_state(&config.field);
                        }

                        //if queue is empty, exit
                        if backlog.len() <= 0 {
                            break;
                        }
                    }
                }
            }
        }

        // if no unflagged mines are remaining, then end game
        //specifically, this checks all the mines and if any are hidden (not flagged) it will continue (i.e. not end the game)
        for (x_pos,y_pos) in mines.iter() {
            if let Some(square) = config.field.get_square_at_mut(*x_pos as isize, *y_pos as isize) {
                if field::State::HIDDEN.eq(square.get_state()) {
                    game_over = false;
                    break;
                }
            }
        }
        if game_over {
            println!("You win, congradulations!");
            // go through every mine and make it visible
            config.field.show_mines();
            cli_utils::print_board(&config.field);
            break;
        }
    }

    //return to main
    Ok(())
}
