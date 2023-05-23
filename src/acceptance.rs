use n_queens::n_queens;

#[test]
fn given_n_1_should_return_1_solution() {
    let result = n_queens(1);

    assert_eq!(result, vec![
        vec!["Q"]
    ])
}

#[test]
#[ignore]
fn given_n_2_should_return_no_solutions() {
    let result = n_queens(2);
    let expected: Vec<Vec<String>> = vec![];

    assert_eq!(result, expected);
}