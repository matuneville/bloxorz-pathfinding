#[cfg(test)]
mod tests {

    extern crate bloxorz_solver;
    use bloxorz_solver::level_solution;

    #[test]
    fn level_1_1() {

        let map: Vec<Vec<isize>> = vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
                                        vec![1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 3],
                                        vec![1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1],
                                        vec![2, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0],
                                        vec![1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0],
                                        vec![0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0],
                                        vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0]];

        let expected_output = 19;

        let result = level_solution(map);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn level_2_1() {

        let map: Vec<Vec<isize>> = vec![vec![1, 1, 1, 1, 0, 0, 0, 0],
                                        vec![1, 1, 1, 1, 0, 1, 1, 1],
                                        vec![2, 0, 0, 1, 0, 1, 3, 1],
                                        vec![0, 0, 0, 1, 1, 1, 1, 1],
                                        vec![0, 0, 0, 1, 1, 1, 1, 1]];

        let expected_output = 10;

        let result = level_solution(map);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn level_2_2() {

        let map: Vec<Vec<isize>> = vec![vec![0, 1, 2, 1],
                                        vec![0, 1, 1, 1],
                                        vec![1, 1, 1, 3],
                                        vec![1, 1, 1, 0]];

        let expected_output = 7;

        let result = level_solution(map);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn level_2_6() {

        let map: Vec<Vec<isize>> = vec![vec![0, 1, 0, 0, 1, 0],
                                        vec![2, 1, 1, 1, 1, 1],
                                        vec![0, 1, 0, 1, 1, 0],
                                        vec![0, 1, 0, 1, 1, 0],
                                        vec![0, 3, 1, 1, 0, 0],
                                        vec![0, 1, 1, 1, 0, 0]];

        let expected_output = 9;

        let result = level_solution(map);
        assert_eq!(result, expected_output);
    }

}
