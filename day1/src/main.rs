use std::io;

fn main() -> io::Result<()> {
    let lines = day1::get_lines_from_filename("input.txt".to_string())?;
    let result = day1::process_lines(lines);
    println!("Calibration Value Sum: {}", result);

    Ok(())
}
