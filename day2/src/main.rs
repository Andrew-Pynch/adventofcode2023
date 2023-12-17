use std::io;

fn main() -> io::Result<()> {
    let lines = day2::get_lines_from_filename("input.txt".to_string())?;
    let result = day2::process_lines(lines);
    println!("Number of possible games: {}", result);

    Ok(())
}
