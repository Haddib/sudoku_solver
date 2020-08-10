use std::io::Read;

//---------------CELL---------------

struct Cell {
    value: i32,
    row_number: usize,
    column_number: usize,
    box_number: usize,
    excluded_numbers: Vec<i32>
}

impl Cell {
    fn new(value: i32, row_number: usize, column_number: usize, box_number: usize) -> Self {
        Self{
            value,
            row_number,
            column_number,
            box_number,
            excluded_numbers: Vec::new()
        }
    }

    // has this number been tried before in this cell?
    fn is_number_excluded(self: &Self, value: i32) -> bool{
        self.excluded_numbers.contains(&value)
    }

    // the current number is probably blocking the solution
    fn add_to_excluded_numbers(self: &mut Self){
        self.excluded_numbers.push(self.value);
    }

    //the problem is not this cell, clear the tried numbers.
    fn clear_excluded_numbers(self: &mut Self){
        self.excluded_numbers.clear();
    }
}
//---------------MAIN---------------

fn main() {
    
    //points to the corresponding row (litarally just the index...)
    let row_indicies: Vec<usize> = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8,
        9,10,11,12,13,14,15,16,17,
        18,19,20,21,22,23,24,25,26,
        27,28,29,30,31,32,33,34,35,
        36,37,38,39,40,41,42,43,44,
        45,46,47,48,49,50,51,52,53,
        54,55,56,57,58,59,60,61,62,
        63,64,65,66,67,68,69,70,71,
        72,73,74,75,76,77,78,79,80
    ];
    
    // points to the corresponding columns (read top > down)
    let column_indicies: Vec<usize> = vec![
        0, 9,18,27,36,45,54,63,72,
        1,10,19,28,37,46,55,64,73,
        2,11,20,29,38,47,56,65,74,
        3,12,21,30,39,48,57,66,75,
        4,13,22,31,40,49,58,67,76,
        5,14,23,32,41,50,59,68,77,
        6,15,24,33,42,51,60,69,78,
        7,16,25,34,43,52,61,70,79,
        8,17,26,35,44,53,62,71,80
    ];

    // points to the corresponding box
    let box_indicies: Vec<usize> = vec![
        0, 1, 2, 9,10,11,18,19,20,
        3, 4, 5,12,13,14,21,22,23,
        6, 7, 8,15,16,17,24,25,26,
        27,28,29,36,37,38,45,46,47,
        30,31,32,39,40,41,48,49,50,
        33,34,35,42,43,44,51,52,53,
        54,55,56,63,64,65,72,73,74,
        57,58,59,66,67,68,75,76,77,
        60,61,62,69,70,71,78,79,80
    ];

    let mut current_index = 0;
    let init_board: Vec<i32> = read_puzzle();
    let mut board = read_cells_into_board(&init_board); 

    println!("Original puzzle");
    print_board(&board);

    while current_index < 80 {
        // is this cell predetermined
        if init_board[current_index] != 0 {
            //go to the next cell
            current_index += 1;
        } 
        else {
            //check for every value 
            for val in 1..=9 {
                // is the value unique AND hasn't been tried before?
                if is_valid_and_unique((&row_indicies, &column_indicies, &box_indicies), current_index, val, &board){
                    // if it is, assign this cell this value and go to the next cell
                    board[current_index].value = val;
                    current_index += 1;
                    break;
                }
                else if val == 9 {
                    //if we made it here, no value fits this cell
                    //reset the values tried in this cell
                    board[current_index].clear_excluded_numbers();
                    // go to the previous cell
                    current_index -= 1;
                    // find the previous editable cell
                    while init_board[current_index] != 0 {
                        if current_index == 0 {
                            //if there is no editable cell, the puzzle in unsolvable
                            println!("Puzzle unsolvable :(");
                            return;
                        }
                        current_index -= 1;
                    }
                    //if we found an editable cell, add it's current value to the list of already tried values.
                    board[current_index].add_to_excluded_numbers();
                    board[current_index].value = 0;
                }    
            }
        }
    }
    println!("Solved!");
    print_board(&board);
}

//------------FUNCTIONS-------------

fn read_puzzle() -> Vec<i32> {
    let mut file = std::fs::File::open("puzzle.txt").unwrap();
    let mut ascii_buffer = Vec::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut ascii_buffer).unwrap();

    for i in ascii_buffer {
        if i < 58 && i >= 48 {
            buffer.push((i - 48) as i32);
        }
    }
    if buffer.len() != 81 {
        println!("ERROR! puzzle.txt should have 81 numbers but, actually has {}", buffer.len());
        std::process::exit(0);
    }
    buffer
}


// creates a vector of cells from a vector of ints
fn read_cells_into_board(init_board: &[i32]) -> Vec<Cell>{
    let mut board: Vec<Cell> = Vec::new();
    for row in 0..9{
        for column in 0..9 {
            let i = row * 9 + column;
            board.push(Cell::new(init_board[i], row, column, get_assigned_box(row, column)));
        }
    } 
    board
}

// checks if a value is unique in its row, column, box AND if it has been tried before.
fn is_valid_and_unique((rows, columns, boxes): (&[usize], &[usize], &[usize]), current_index: usize, value: i32, board: &[Cell]) -> bool {
    !board[current_index].is_number_excluded(value) && 
            is_unique(board[current_index].row_number, &rows, value, current_index, &board) &&
            is_unique(board[current_index].column_number, &columns, value, current_index, &board) &&
            is_unique(board[current_index].box_number, &boxes, value, current_index, &board)
}

//checks if the value is unique in this specific row/column/box
fn is_unique(container_index: usize, container: &[usize], value: i32, current_index: usize, board: &[Cell]) -> bool {
    let mut current_container: Vec<usize> = Vec::new();
    for i in 0..9{
        current_container.push(container[container_index * 9 + i]);
    }
    for j in current_container{
        if j != current_index && board[j].value == value{
            return false;
        }
    }
    true
}

// check what box this cell belongs to based on the row and column.
fn get_assigned_box(row: usize, column: usize) -> usize {
    if row < 3 {
        if column < 3 { 0 } else if column > 5 { 2 } else { 1 }
    } 
    else if row > 5 {
        if column < 3 { 6 } else if column > 5 { 8 } else { 7 }
    } 
    else if column < 3 { 3 } else if column > 5 { 5 } else { 4 }
}

fn print_board(board: &[Cell]){
    for i in 0..9 {
        for j in 0..9{
            if board[i*9+j].value == 0 {
                print!("  ");
            }
            else{
                print!("{} ", board[i*9+j].value);
            }
            if (j + 1) % 3 == 0 && j != 8 {
                print!("| ")
            }
        }
        if (i + 1) % 3 == 0 && i != 8 {
            println!("\n------+-------+------");
        }
        else {
            println!();
        }
    }
    println!();
}
