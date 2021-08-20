// *.......
// ..*.....
// ....*...
// ......*.
// .*......
// .......*
// .....*..
// ...*....

use std::io::{stdin, BufReader, BufRead};

fn main() {
    let board: Vec<Vec<bool>> = BufReader::new(stdin()).lines().
        map(|l| l.unwrap().chars().into_iter().map(|c| c=='*').collect() ).collect();

    for (row_index, row) in board.iter().enumerate() {
        for (column_index, &cell) in row.iter().enumerate() {
            if !cell {
                continue
            }

            // check row
            let mut one_queen = false;

            for c in 0..row.len() {
                if row[c] {
                    if one_queen {
                        println!("invalid");
                        return;
                    }
                    one_queen = true;
                }
            }

            // Check column
            one_queen = false;

            for r in 0..board.len() {
                if board[r][column_index] {
                    if one_queen {
                        println!("invalid");
                        return;
                    }
                    one_queen = true;
                }
            }

            // Check diagonal
            let mut row_start_index = (column_index-row_index).max(0);
            let mut column_start_index = (row_index-column_index).max(0);

            one_queen = false;
            while row_start_index < row.len() && column_start_index < board.len() {
                if board[row_start_index][column_start_index] {
                    if one_queen {
                        println!("invalid");
                        return;
                    }
                    one_queen = true;
                }
                row_start_index += 1;
                column_start_index += 1;
            }
        }
    }
}
