use std::{
    error::Error, //better errors
    io::{self, Write}, //io interactions
    fmt::Display, str::FromStr //traits
};

use super::field::Field;

//DATA
const FLAG:char = '\u{f024}';

/// resets the screen
pub fn reset_screen() {
    //clear screen and position cursor to row 1 column 1
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

/// prints the board state to screen
pub fn print_game_state(field: &Field) {
    //print header
    print_header(field.get_n_mines(), field.get_dimensions());
    //print board
    print_board(field);
    //print instrustions for inputting commands
    print_command_instructions();
}
/// prints header of board state
fn print_header(n_mines:usize, board_size:u8) {
    let width = board_size as usize + 3;
    println!(
"{title:^width$}

{flagText:^width$}",
    title = "MINESWEEPER" as &str,
    flagText = format!("{}: {:0>3}",FLAG,n_mines),
    );
}
/// prints board
pub fn print_board(field: &Field) {
    println!(
"  #{column_letters}
{grid}",
        column_letters = (0u8..field.get_dimensions()).fold(String::new(), |mut acc, i| {acc.push((i+97) as char); acc}),
        grid = field.get_grid().iter().enumerate().fold(String::new(), //fold contents of grid into a single string
            |mut acc, row_tup| { //for every row
                //DATA 
                let num = row_tup.0;
                let row = row_tup.1;

                //add the row number, and contents to acc
                acc.push_str(
                    format!(
                        "{:<2}#{}",
                        num, //line numbers
                        row.iter().fold(String::new(), |mut nested_acc, square| {
                            nested_acc.push(square.get_icon());
                            return nested_acc;
                        }), //fold the contents of the row into a single string
                    ).as_str()
                );

                //newline
                acc.push('\n');

                return acc;
            }
        ),
    )
}

fn print_command_instructions() {println!(
"
Commands:
 - Check (prefix: 'C'): check if the following square is a mine or not
                        you lose the game is it's a mine
 - Flag (prefix: 'F'):  adds a flag to the following square, 
                        or removes it if one is already there
The format for commands is as follows:
{{command prefix}}{{column letter}}{{row number}}

For example, the command Fa0 would add a flag to the top left corner.

"
);}

/// gets a string from user input
pub fn get_string_from_user_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    //DATA
    let mut raw_input = String::new();

    //print prompt
    print!("{}", prompt);
    //make sure it's printed before getting input
    io::stdout().flush().expect("couldn't flush stdout");

    //read user input from standard input, and store it to raw_input, then return it or an error as needed
    raw_input.clear(); //clear input
    match io::stdin().read_line(&mut raw_input) {
        Ok(_num_bytes_read) => return Ok(String::from(raw_input.trim())),
        Err(err) => return Err(format!("ERROR: CANNOT READ INPUT!: {}", err).into()),
    }
}
/// generic function to get a number from the passed string (user input)
/// pass a min lower  than the max to have minimun and maximun bounds
/// pass a min higher than the max to only have a minumum bound
/// pass a min equal   to  the max to only have a maximun bound
/// 
/// Errors:
/// no number on user input
pub fn _get_number_from_input<T:Display + PartialOrd + FromStr>(prompt: &str, min:T, max:T) -> Result<T, Box<dyn Error>> {
    //DATA
    let raw_input: String;
    let processed_input: String;

    
    //input looop
    raw_input = loop {
        match get_string_from_user_input(prompt) {
            Ok(input) => break input,
            Err(e) => {
                eprintln!("{}",e);
                continue;
            },
        }
    };

    //filter out num-numeric characters from user input
    processed_input = raw_input.chars().filter(|c| c.is_numeric()).collect();

    //from input, try to read a number
    match processed_input.trim().parse() {
        Ok(i) => {
            //what bounds must the input fall into
            if min < max {  //have a min and max bound: [min,max]
                if i >= min && i <= max {//is input valid, within bounds
                    return Ok(i); //exit the loop with the value i, returning it
                } else { //print error message specific to this case
                    return Err(format!("ONLY BETWEEN {} AND {}, PLEASE!", min, max).into());
                } 
            } else if min > max { //only a min bound: [min, infinity)
                if i >= min {
                    return Ok(i);
                } else {
                    return Err(format!("NO LESS THAN {}, PLEASE!", min).into());
                }
            } else { //only a max bound: (-infinity, max]
                if i <= max {
                    return Ok(i);
                } else {
                    return Err(format!("NO MORE THAN {}, PLEASE!", max).into());
                }
            }
        },
        Err(_e) => return Err(format!("Error: couldn't find a valid number in {}",raw_input).into()),
    }
}
