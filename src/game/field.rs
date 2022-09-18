use rand::{Rng, prelude::thread_rng}; //rng

//DATA
const HIDDEN: char = '-';
const FLAGGED: char= 'f';
const VISIBLE: &str= " 12345678";
const MINE: char = '*';

/**
 * handles the game field, a 26x26 grid of squares, each square is either a mine or not a mine, and has one of 3 states:
 * -visible
 * -hidden
 * -flagged
 */

pub struct Field {
    grid: Vec<Vec<Square>>,
    difficulty: Difficulty,
    mines: Vec<(u8,u8)>,
    n_mines:usize,
    n_flags:usize,
}
impl Field {
    /// creates new empty field
    pub fn new() -> Field {
        let field:Field = Field { 
            grid: Vec::new(),
            difficulty: Difficulty::BEGINNER, //default value, changed in populate
            mines: Vec::new(),
            n_mines: 0,
            n_flags: 0,
        };

        return field
    }
    /// populates the field as a FIELD_DIMENSIONS x FIELD_DIMENSIONS grid with num_mines mines
    pub fn populate(&mut self, difficulty:Difficulty) {
        //update difficulty
        self.difficulty = difficulty;
        //DATA
        let dimensions = self.difficulty.get_dimensions();
        let mut rng = thread_rng();

        //populate field with sqaures
        for row in 0..dimensions {
            //create new row
            let mut new_row: Vec<Square> = Vec::new();

            //fill it with squares
            for col in 0..dimensions {
                //DATA
                let mut curr_square:Square = Square::new(col,row,rng.gen_bool(self.difficulty.p_is_mine()));

                //if it's a mine, update mine count and increase the danger of each adjactent square
                if curr_square.is_mine() {
                    self.n_mines += 1; //update mine count
                    //add mine position to mines
                    self.mines.push((col,row));
                    
                    //update north square
                    if let Some(square) = self.get_square_at_mut(col as isize, row as isize - 1) {
                        square.danger += 1;
                    }
                    //update west square
                    if col > 0 {//have to do it different because the row is still being built
                        if let Some(square) = new_row.get_mut(col as usize -1) {
                            square.danger += 1;
                        }
                    }
                    //update northwest square
                    if let Some(square) = self.get_square_at_mut(col as isize - 1, row as isize - 1) {
                        square.danger += 1;
                    }
                    //update northeast square
                    if let Some(square) = self.get_square_at_mut(col as isize + 1, row as isize - 1) {
                        square.danger += 1;
                    }
                } 
                //otherwise set danger to the number of adjacent mines
                else {
                    //check north square
                    if let Some(square) = self.get_square_at(col as isize, row as isize - 1) {
                        if square.is_mine() {
                            curr_square.danger += 1;
                        }
                    }
                    //check west square
                    if col > 0 { //have to do it different because the row is still being built
                        if let Some(square) = new_row.get_mut(col as usize -1) {
                            if square.is_mine() {
                                curr_square.danger += 1;
                            }
                        }
                    }
                    //check northwest square
                    if let Some(square) = self.get_square_at(col as isize - 1, row as isize - 1) {
                        if square.is_mine() {
                            curr_square.danger += 1;
                        }
                    }
                    //check northeast square
                    if let Some(square) = self.get_square_at(col as isize + 1, row as isize - 1) {
                        if square.is_mine() {
                            curr_square.danger += 1;
                        }
                    }
                }
                
                //make new square and add it to row
                new_row.push(curr_square);
            }
            
            //add row to field
            self.grid.push(new_row);
        }
    }

    //makes all the mines visible
    pub fn show_mines(&mut self) {
        let mines = self.mines.clone();
        for (x_pos,y_pos) in mines.iter() {
            if let Some(sqr) = self.get_square_at_mut(*x_pos as isize, *y_pos as isize) {
                if State::HIDDEN.eq(sqr.get_state()) {
                    sqr.set_state(State::VISIBLE);
                }
            }
        }
    }


    pub fn check_and_update_states_of_adjacent_squares(&mut self, x_pos:isize,y_pos:isize) -> Vec<(isize,isize)> {
        let mut backlog:Vec<(isize,isize)> = Vec::new();

        //N
        if let Some(tmp_sqr) = self.get_square_at_mut(x_pos, y_pos-1) {
            if tmp_sqr.get_danger() == 0 && State::HIDDEN.eq(tmp_sqr.get_state()) {backlog.push((x_pos,y_pos-1))}
            tmp_sqr.set_state(State::VISIBLE);
        }
        //NE
        if let Some(tmp_sqr) = self.get_square_at_mut(x_pos+1, y_pos-1) {
            if tmp_sqr.get_danger() == 0 && State::HIDDEN.eq(tmp_sqr.get_state()) {backlog.push((x_pos+1, y_pos-1))}
            tmp_sqr.set_state(State::VISIBLE);
        }
        //E
        if let Some(tmp_sqr) = self.get_square_at_mut(x_pos+1, y_pos) {
            if tmp_sqr.get_danger() == 0 && State::HIDDEN.eq(tmp_sqr.get_state()) {backlog.push((x_pos+1, y_pos))}
            tmp_sqr.set_state(State::VISIBLE);
        }
        //SE
        if let Some(tmp_sqr) = self.get_square_at_mut(x_pos+1, y_pos+1) {
            if tmp_sqr.get_danger() == 0 && State::HIDDEN.eq(tmp_sqr.get_state()) {backlog.push((x_pos+1, y_pos+1))}
            tmp_sqr.set_state(State::VISIBLE);
        }
        //S
        if let Some(tmp_sqr) = self.get_square_at_mut(x_pos, y_pos+1) {
            if tmp_sqr.get_danger() == 0 && State::HIDDEN.eq(tmp_sqr.get_state()) {backlog.push((x_pos, y_pos+1))}
            tmp_sqr.set_state(State::VISIBLE);
        }
        //SW
        if let Some(tmp_sqr) = self.get_square_at_mut(x_pos-1, y_pos+1) {
            if tmp_sqr.get_danger() == 0 && State::HIDDEN.eq(tmp_sqr.get_state()) {backlog.push((x_pos-1, y_pos+1))}
            tmp_sqr.set_state(State::VISIBLE);
        }
        //W
        if let Some(tmp_sqr) = self.get_square_at_mut(x_pos-1, y_pos) {
            if tmp_sqr.get_danger() == 0 && State::HIDDEN.eq(tmp_sqr.get_state()) {backlog.push((x_pos-1, y_pos))}
            tmp_sqr.set_state(State::VISIBLE);
        }
        //NW
        if let Some(tmp_sqr) = self.get_square_at_mut(x_pos-1, y_pos-1) {
            if tmp_sqr.get_danger() == 0 && State::HIDDEN.eq(tmp_sqr.get_state()) {backlog.push((x_pos-1, y_pos-1))}
            tmp_sqr.set_state(State::VISIBLE);
        }

        backlog
    }

    //getters
    /// get grid
    pub fn get_grid(&self) -> &Vec<Vec<Square>> {&self.grid}
    /// get n_mines
    pub fn get_n_mines(&self) -> usize {self.n_mines}
    /// get n_flags
    pub fn get_n_flags(&self) -> usize {self.n_flags}
    /// get mines
    pub fn get_mines(&self) -> &Vec<(u8,u8)> {&self.mines}
    /// get a reference to the square at the given x and y coordinate
    pub fn get_square_at(&self, x_pos:isize, y_pos:isize) -> Option<&Square> { // it accepts negative values so that it can handle cases where code is checking squares on the border without the need for additional logic
        if x_pos >= 0 && y_pos >= 0 {
            if let Some(row) = self.grid.get(y_pos as usize) {
                if let Some(square) = row.get(x_pos as usize) {
                    return Some(square);
                }
            }
        }
        return None;
    }
    /// get a mutable reference to the square at the given x and y coordinate
    pub fn get_square_at_mut(&mut self, x_pos:isize, y_pos:isize) -> Option<&mut Square> {
        if x_pos >= 0 && y_pos >= 0 { 
            if let Some(row) = self.grid.get_mut(y_pos as usize) {
                if let Some(square) = row.get_mut(x_pos as usize) {
                    return Some(square);
                }
            }
        }
        return None;
    }
    /// get field dimensions from difficulty
    pub fn get_dimensions(&self) -> u8 {self.difficulty.get_dimensions()}

    //incrementers and decrementers
    ///increments n_flags
    pub fn increment_n_flags(&mut self) {self.n_flags += 1}
    ///decrements n_flags
    pub fn decrement_n_flags(&mut self) {self.n_flags -= 1}
}

/// a single square on a grid
pub struct Square {
    x_pos:u8,
    y_pos:u8,
    is_mine:bool,
    danger:usize,
    state: State,
}
impl Square {
    /// create new square
    pub fn new(x_pos:u8,y_pos:u8,is_mine:bool) -> Square {
        return Self { is_mine, state: State::HIDDEN, x_pos, y_pos, danger: 0}
    }
    /// return icon associated with the squares state
    pub fn get_icon(&self) -> char {
        return match self.state {
            State::HIDDEN => HIDDEN,
            State::VISIBLE => {if self.is_mine {MINE} else {VISIBLE.chars().nth(self.danger).unwrap()}},
            State::FLAGGED => FLAGGED,
        }
    }
    //getters and setters
    /// returns if it's a mine
    pub fn is_mine(&self) -> bool {self.is_mine}
    /// return the position of the square as (x,y), starting from top left corner
    pub fn get_position(&self) -> (u8,u8) {
        return (self.x_pos,self.y_pos);
    }
    /// get the danger (number of mines surrounding the square)
    pub fn get_danger(&self) -> usize {self.danger}
    /// get the squares current state
    pub fn get_state(&self) -> &State {&self.state}
    /// set the squares state
    pub fn set_state(&mut self, state:State) {self.state = state}
}

#[derive(PartialEq)]
pub enum State {
    HIDDEN,
    VISIBLE,
    FLAGGED,
}

//#[derive(Clone, Copy)]
pub enum Difficulty {
    BEGINNER,
    INTERMEDIATE,
    ADVANCED,
}
impl Difficulty {
    /// dimensions of field based on difficulty 
    pub fn get_dimensions(&self) -> u8 {
        return match *self {
            Difficulty::BEGINNER => 9,
            Difficulty::INTERMEDIATE => 16,
            Difficulty::ADVANCED => 24,
        }
    }
    /// probability a square is a mine based on difficulty
    pub fn p_is_mine(&self) -> f64 {
        return match *self {
            Difficulty::BEGINNER => 0.12, // 10/81
            Difficulty::INTERMEDIATE => 0.15, // 40/256
            Difficulty::ADVANCED => 0.17, // 99/576
        }
    }
}
