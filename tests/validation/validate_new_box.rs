#[cfg(test)]
mod validate_new_box {
    use sabita::core::grid::BoxLocation;
    use sabita::core::validation::validate_new_box;

    #[test]
    fn valid_1() {
        let testing_this = 1;

        let matrix = vec![
            vec![7, 8, 2, 9, 4, 6, 5, 3, 1],
            vec![4, testing_this, 0, 0, 0, 0, 0, 0, 0],
            vec![5, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![6, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![8, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![3, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![9, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let is_valid = validate_new_box(
            &matrix,
            &BoxLocation {
                line: 1,
                column: 1,
                region: 0,
            },
        );

        match is_valid {
            Ok(_) => assert!(true),
            Err(err) => panic!("Should not throw error {}", err),
        }
    }

    #[test]
    fn valid_2() {
        let testing_this = 1;

        let matrix = vec![
            vec![7, 8, 2, 9, 4, 6, 5, 3, 1],
            vec![4, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![5, 0, testing_this, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![6, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![8, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![3, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![9, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let is_valid = validate_new_box(
            &matrix,
            &BoxLocation {
                line: 2,
                column: 2,
                region: 0,
            },
        );

        match is_valid {
            Ok(_) => assert!(true),
            Err(err) => panic!("Should not throw error {}", err),
        }
    }

    #[test]
    fn invalid() {
        let testing_this = 5;

        let matrix = vec![
            vec![7, 8, 2, 9, 4, 6, 5, 3, 1],
            vec![4, testing_this, 0, 0, 0, 0, 0, 0, 0],
            vec![5, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![6, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![8, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![3, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![9, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let is_valid = validate_new_box(
            &matrix,
            &BoxLocation {
                line: 2,
                column: 2,
                region: 0,
            },
        );

        match is_valid {
            Ok(_) => panic!("Should not be valid"),
            Err(err) => assert_eq!(
                err.to_string(),
                "Region index 0 is not valid, duplicate value 5"
            ),
        }
    }
}
