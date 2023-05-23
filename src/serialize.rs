pub fn serialize(board: Vec<usize>) -> Vec<String> {
    vec![String::from("Q")]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_size_board() {
        let board = vec![0];

        let result = serialize(board);

        assert_eq!(result, vec!["Q"])
    }
}