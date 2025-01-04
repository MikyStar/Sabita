#[cfg(test)]
mod get_box_solutions {
    use sabita::core::constants::TO_BE_SOLVED;
    use sabita::core::grid::BoxLocation;
    use sabita::core::solver::get_box_solutions;

    #[test]
    fn should_be_1() {
        let values = vec![
            vec![3, 9, 1, 2, 8, 6, 5, 7, 4],
            vec![4, 8, 7, 3, 5, 9, 1, 2, 6],
            vec![6, 5, 2, 7, 1, 4, 8, 3, 9],
            vec![8, 7, 5, 4, 3, 1, 6, 9, 2],
            vec![2, 1, 3, 9, 6, 7, 4, 8, 5],
            vec![9, 6, 4, 5, 2, 8, 7, 1, 3],
            vec![1, 4, 9, 6, 7, 3, 2, 5, 8],
            vec![5, 3, 8, 1, 4, 2, 9, 6, 7],
            vec![7, 2, 6, 8, 9, 5, 3, 4, TO_BE_SOLVED],
        ];

        let location = BoxLocation {
            line: 8,
            column: 8,
            region: 8,
        };

        let answers = get_box_solutions(&values, &location);

        match answers {
            Ok(res) => assert_eq!(res, vec![1]),
            Err(err) => panic!("An error was catched: {}", err),
        }
    }

    #[test]
    fn should_be_3() {
        let values = vec![
            vec![TO_BE_SOLVED, 9, 1, 2, 8, 6, 5, 7, 4],
            vec![4, 8, 7, 3, 5, 9, 1, 2, 6],
            vec![6, 5, 2, 7, 1, 4, 8, 3, 9],
            vec![8, 7, 5, 4, 3, 1, 6, 9, 2],
            vec![2, 1, 3, 9, 6, 7, 4, 8, 5],
            vec![9, 6, 4, 5, 2, 8, 7, 1, 3],
            vec![1, 4, 9, 6, 7, 3, 2, 5, 8],
            vec![5, 3, 8, 1, 4, 2, 9, 6, 7],
            vec![7, 2, 6, 8, 9, 5, 3, 4, 1],
        ];

        let location = BoxLocation {
            line: 0,
            column: 0,
            region: 0,
        };

        let answers = get_box_solutions(&values, &location);

        match answers {
            Ok(res) => assert_eq!(res, vec![3]),
            Err(err) => panic!("An error was catched: {}", err),
        }
    }

    #[test]
    fn should_be_3_or_8() {
        let testing_this_box = TO_BE_SOLVED;

        let values = vec![
            vec![testing_this_box, 9, 1, 2, TO_BE_SOLVED, 6, 5, 7, 4],
            vec![4, TO_BE_SOLVED, 7, 3, 5, 9, 1, 2, 6],
            vec![6, 5, 2, 7, 1, 4, 8, 3, 9],
            vec![TO_BE_SOLVED, 7, 5, 4, 3, 1, 6, 9, 2],
            vec![2, 1, 3, 9, 6, 7, 4, 8, 5],
            vec![9, 6, 4, 5, 2, 8, 7, 1, 3],
            vec![1, 4, 9, 6, 7, 3, 2, 5, 8],
            vec![5, 3, 8, 1, 4, 2, 9, 6, 7],
            vec![7, 2, 6, 8, 9, 5, 3, 4, 1],
        ];

        let location = BoxLocation {
            line: 0,
            column: 0,
            region: 0,
        };

        let answers = get_box_solutions(&values, &location);

        let should_be = vec![3, 8];

        match answers {
            Ok(res) => assert_eq!(res, should_be),
            Err(err) => panic!("An error was catched: {}", err),
        }
    }

    #[test]
    fn should_get_error() {
        let faulty_value = 3; // Should actually be 4 for the function not to break

        let values = vec![
            vec![TO_BE_SOLVED, 9, 1, 2, 8, 6, 5, 7, 4],
            vec![faulty_value, 8, 7, 3, 5, 9, 1, 2, 6],
            vec![6, 5, 2, 7, 1, 4, 8, 3, 9],
            vec![8, 7, 5, 4, 3, 1, 6, 9, 2],
            vec![2, 1, 3, 9, 6, 7, 4, 8, 5],
            vec![9, 6, 4, 5, 2, 8, 7, 1, 3],
            vec![1, 4, 9, 6, 7, 3, 2, 5, 8],
            vec![5, 3, 8, 1, 4, 2, 9, 6, 7],
            vec![7, 2, 6, 8, 9, 5, 3, 4, 1],
        ];

        let location = BoxLocation {
            line: 0,
            column: 0,
            region: 0,
        };

        let answers = get_box_solutions(&values, &location);

        match answers {
            Ok(_) => assert!(false, "Should have thrown error"),
            Err(err) => assert_eq!(
                err.to_string(),
                "Solution of the box [0:0](0) could'nt be found"
            ),
        }
    }
}
