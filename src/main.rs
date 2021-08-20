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

    println!("{:?}", board)
}
