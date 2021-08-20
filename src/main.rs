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
            let valid_row = {
                let one_queen = false;

                for c in 0..row.len() {
                    if c {}
                }

                true
            };
        }
    }
}
