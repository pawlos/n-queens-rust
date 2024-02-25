fn is_valid_board(board: Vec<Option<usize>>) -> bool {
    true
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
}