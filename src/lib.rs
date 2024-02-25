mod validity;
mod serialize;

use std::ops::BitOrAssign;
use crate::serialize::serialize_board;
use crate::validity::is_valid_board;

pub fn n_queens(n: usize) -> Vec<Vec<String>> {
    //results
    let mut results: Vec<Vec<Option<usize>>> = vec![];
    //frontier
    let mut frontier: Vec<Vec<Option<usize>>> = vec![];
    //populate the frontier
    for row in 0..n {
        let mut board: Vec<Option<usize>> = (0..n).map(|_| None).collect();
        board[0] = Some(row);
        frontier.push(board);
    }

    //search
    while !frontier.is_empty() {
        let board = frontier.pop().unwrap();

        let number_of_complete_columns = board.iter()
            .filter(|column| column.is_some())
            .count();
        //generate 4 new boards
        for row in 0..n {
            let mut new_board = board.clone();
            new_board[number_of_complete_columns-1] = Some(row);

            if is_valid_board(&new_board) {
                if number_of_complete_columns == n {
                    results.push(new_board);
                } else {
                    frontier.push(new_board);
                }
            }
        }

        // for each new board
        // is it valid and complete add it to result
        // is it valid and not-complete add it to the frontier
    }

    //validity function


     results.iter()
         .map(|board| board.iter().map(|column| column.unwrap()).collect())
         .map(|board| serialize_board(&board))
         .collect()
}

