use std::collections::HashSet;

fn main() {
    let b1 = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let b2 = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let b3 = vec![
        vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
        vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
        vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
        vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
        vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
        vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
        vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
    ];
    println!("{}", is_valid_sudoku(b1));
    println!("{}", is_valid_sudoku(b2));
    println!("{}", is_valid_sudoku(b3));
}

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut row_validator: HashSet<char> = HashSet::new();
    let mut col_validator: HashSet<char> = HashSet::new();
    let mut box_validator: HashSet<char> = HashSet::new();
    for i in 0..9 {
        for j in 0..9 {
            let c = board[i][j];
            if c != '.' && !row_validator.insert(c) {
                return false;
            }
            let c = board[j][i];
            if c != '.' && !col_validator.insert(c) {
                return false;
            }
            let c = board[3 * (i / 3) + (j / 3)][3 * (i % 3) + (j % 3)];
            if c != '.' && !box_validator.insert(c) {
                return false;
            }
        }
        row_validator.clear();
        col_validator.clear();
        box_validator.clear();
    }
    true
}