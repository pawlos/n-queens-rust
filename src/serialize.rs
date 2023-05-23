pub fn serialize(board: &Vec<usize>) -> Vec<String> {
    let mut result = vec![];
    for y in 0..board.len() {
        let mut row = String::new();
        let xx = board.iter()
        .enumerate().filter(|(x,yy)| yy == &&y)
        .last().unwrap().0;
        for x in 0..board.len() {
            if x == xx {
                row.push('Q')
            }
            else {
                row.push('.');
            }
        }
        result.push(row);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_size_board() {
        let board = vec![0];

        let result = serialize(&board);

        assert_eq!(result, vec!["Q"])
    }

    #[test]
    fn four_size_board() {
        let board = vec![1,3,0,2];

        let result = serialize(&board);

        assert_eq!(result, vec!["..Q.",
                                "Q...",
                                "...Q",
                                ".Q.."])
    }
}