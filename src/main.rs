use std::collections::VecDeque;

//---------------CELL---------------

struct Cell {
    value: i32,
    row_number: usize,
    column_number: usize,
    box_number: usize,
    //excluded_numbers: Vec<i32>
}

impl Cell {
    fn new(value: i32, row_number: usize, column_number: usize, box_number: usize) -> Self {
        Self{
            value: value,
            row_number: row_number,
            column_number: column_number,
            box_number: box_number,
          //  excluded_numbers: Vec::new()
        }
    }
}
//---------------MAIN---------------

fn main() {

    let init_board: Vec<i32> = vec![
        5,3,0,  0,7,0,  0,0,0,
        6,0,0,  1,9,5,  0,0,0,
        0,9,8,  0,0,0,  0,6,0,

        8,0,0,  0,6,0,  0,0,3,
        4,0,0,  8,0,3,  0,0,1,
        7,0,0,  0,2,0,  0,0,6,
        
        0,6,0,  0,0,0,  2,8,0,
        0,0,0,  4,1,9,  0,0,5,
        0,0,0,  0,8,0,  0,7,9
        ];
    
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
    let mut board = read_cells_into_board(&init_board); 
    //let boxes = assign_cells_to_boxes(&board);
    //let mut rows: Vec<Container> = assign_cells_to_rows_or_columns(&mut board, true);
    //let mut columns: Vec<Container> = assign_cells_to_rows_or_columns(&mut board, false);

    /*print_board(&board);
    print_boxes(&boxes);
    print_rows(&rows);
    print_rows(&columns);
    */
    while current_index < 80 {
        // is this cell predetermined
        if init_board[current_index] != 0 {
            println!("Value at {} is immutable", current_index);
            //go to the next cell
            current_index += 1;
        } else {
            //check for every value ABOVE the current value
            for val in board[current_index].value..=9 {
                if val == 0 {
                    board[current_index].value = 1;
                } else {
                    board[current_index].value = val;
                }
                if is_unique(board[current_index].row_number, &row_indicies, board[current_index].value, current_index, &board) &&
                    is_unique(board[current_index].column_number, &column_indicies, board[current_index].value, current_index, &board) &&
                    is_unique(board[current_index].box_number, &box_indicies, board[current_index].value, current_index, &board)
                {
                    println!("{} is unique", board[current_index].value);
                    current_index += 1;
                    break;
                } else if val == 9 {
                    board[current_index].value = 0;
                    if current_index == 0 {
                        return;
                    } else {
                        current_index -= 1;
                        if board[current_index].value != 9 {
                            board[current_index].value += 1;
                        }
                        while board[current_index].value == 9 {
                            board[current_index].value = 0;
                            current_index -= 1;
                        }
                        break;
                    }
                } 
            }
        }
        println!("current index: {}", current_index);
        
        print_board(&board);
    }
}

//------------FUNCTIONS-------------

fn read_cells_into_board(init_board: &Vec<i32>) -> VecDeque<Cell>{
    let mut board: VecDeque<Cell> = VecDeque::new();
    for row in 0..9{
        for column in 0..9 {
            let i = row * 9 + column;
            board.push_back(Cell::new(init_board[i], row, column, get_assigned_box(row, column)));
        }
    } 
    return board;
}

fn is_unique(container_index: usize, container: &Vec<usize>, value: i32, current_index: usize, board: &VecDeque<Cell>) -> bool {
    let mut current_container: Vec<usize> = Vec::new();
    for i in 0..9{
        current_container.push(container[container_index * 9 + i]);
    }
    for i in current_container{
        if i != current_index && board[i].value == value{
            return false
        }
    }
    return true;
}

/*fn is_unique(row_index: usize, column_index: usize, box_index: usize, value: i32, current_index: usize, board: &VecDeque<Cell>) -> bool {
    let mut current_row: Vec<usize> = Vec::new();
    let mut current_column: Vec<usize> = Vec::new();
    let mut current_box: Vec<usize> = Vec:: new();
    for i in 0..9{
        current_row.push(ROW_INDICIES[row_index * 9 + i]);
        current_column.push(COLUMN_INDICIES[column_index * 9 + i]);
        current_box.push(BOX_INDICIES[box_index * 9 + i]);
    }  
    for i in current_row{
        if i != current_index && board[i].value == value{
            return false
        }
    }
    for i in current_column{
        if i != current_index && board[i].value == value{
            return false
        }
    }
    for i in current_box{
        if i != current_index && board[i].value == value{
            return false
        }
    }
    return true;
}*/

fn get_assigned_box(row: usize, column: usize) -> usize {
    if row < 3 {
        if column < 3 {
            return 0;
        } else if column > 5 {
            return 2;
        } else {
            return 1;
        }
    } else if row > 5 {
        if column < 3 {
            return 6;
        } else if column > 5 {
            return 8;
        } else {
            return 7;
        }
    } else {
        if column < 3 {
            return 3;
        } else if column > 5 {
            return 5;
        } else {
            return 4;
        }
    }
}

//-----------------------DEBUG------------------------

fn print_board(board: &VecDeque<Cell>){
    for i in 0..9 {
        for j in 0..9{
            print!("{}, ", board[i*9+j].value);
        }
        println!("");
    }
    println!("");
}
