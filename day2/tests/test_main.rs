#[cfg(test)]
mod tests {
    use day2::{get_lines_from_filename, process_lines};
    use std::io;

    #[test]
    fn test_process_lines() -> io::Result<()> {
        let lines = get_lines_from_filename("example_input.txt".to_string())?;
        let result = process_lines(lines);

        let expected_possible_games = 8;
        assert_eq!(result, expected_possible_games);

        Ok(())
    }
}
