// *.......
// ..*.....
// ....*...
// ......*.
// .*......
// .......*
// .....*..
// ...*....

use std::io::{BufRead, BufReader, stdin};

fn main() {
    let board: Vec<Vec<i32>> = BufReader::new(stdin()).lines().
        map(|l| l.unwrap().chars().into_iter().map(|c| if c == '*' { 1 } else { 0 }).collect()).collect();

    let transpose_board: Vec<Vec<i32>> = transpose(&board);

    println!("From queen point of view: {}", to_string(
        from_queen_point_of_view(&board) && from_queen_point_of_view(&transpose_board)
    ));

    println!("Calculate sum: {}", to_string(
        calculate_rows(&board) && calculate_rows(&transpose_board)
    ));
}

fn to_string(b: bool) -> String {
    if b {
        return String::from("valid");
    }

    String::from("invalid")
}

fn transpose(board: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut transposed: Vec<Vec<i32>> = vec![vec! {}; board.len()];

    for r in board.iter() {
        for (i, &v) in r.iter().enumerate() {
            transposed[i].push(v);
        }
    }

    transposed
}

fn from_queen_point_of_view(board: &Vec<Vec<i32>>) -> bool {
    for (row_index, row) in board.iter().enumerate() {
        for (column_index, &cell) in row.iter().enumerate() {
            if cell != 1 {
                continue;
            }

            // check row
            let mut one_queen = false;

            for c in 0..row.len() {
                if row[c] == 1 {
                    if one_queen {
                        return false;
                    }
                    one_queen = true;
                }
            }

            // Check right diagonal
            let mut row_start_index = (row_index as i32 - column_index as i32).max(0) as usize;
            let mut column_start_index = (column_index as i32 - row_index as i32).max(0) as usize;

            one_queen = false;
            while row_start_index < row.len() && column_start_index < board.len() {
                if board[row_start_index][column_start_index] == 1 {
                    if one_queen {
                        return false;
                    }
                    one_queen = true;
                }
                row_start_index += 1;
                column_start_index += 1;
            }
        }
    }

    true
}

fn calculate_rows(board: &Vec<Vec<i32>>) -> bool {
    // row
    for r in board.iter() {
        if r.iter().sum::<i32>() > 1 {
            return false;
        }
    }

    // diagonal
    for i in 0..board.len() {
        let mut count = 0;
        let mut index = i;

        for j in 0..board.len()-i {
            count += board[index][j];
            if count > 1 {
                return false;
            }

            index += 1;
        }
    }

    for j in 0..board[0].len() {
        let mut count = 0;
        let mut jindex = j;

        for i in 0..board.len()-j {
            count += board[i][jindex];
            if count > 1 {
                return false;
            }

            jindex += 1;
        }
    }

    return true;
}