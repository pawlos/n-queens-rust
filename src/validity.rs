fn is_valid_board(board: Vec<Option<usize>>) -> bool {
    let mut result = true;

    let number_complete_columns = board.iter().filter(|column| column.is_some())
        .count();

    let newest_queen_position = board[number_complete_columns - 1].unwrap();
    for x in 0..number_complete_columns-1 {
        if board[x].unwrap() == newest_queen_position {
            result = false;
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_tile_board_is_valid() {
        let board = vec![Some(0)];

        let result = is_valid_board(board);

        assert_eq!(result, true);
    }

    #[test]
    fn given_horizontally_adjacent_queens_should_return_invalid() {
        let board = vec![Some(1), Some(3), Some(0), Some(0)];

        let result = is_valid_board(board);

        assert_eq!(result, false)
    }

    #[test]
    fn given_far_horizontally_adjacent_queens_on_incomplete_board_should_return_invalid() {
        let board = vec![Some(1), Some(3), Some(1), None];

        let result = is_valid_board(board);

        assert_eq!(result, false)
    }

    #[test]
    fn given_complete_valid_board_is_should_return_valid() {
        let board = vec![Some(1), Some(3), Some(0), Some(2)];

        let result = is_valid_board(board);

        assert_eq!(result, true)
    }

    #[test]
    #[ignore]
    fn given_diagonally_adjacent_queens_should_return_invalid() {
        let board = vec![Some(0), Some(1), None, None];

        let result = is_valid_board(board);

        assert_eq!(result, false)
    }
}