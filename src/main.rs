extern crate galactic_merchants_guide as guide;
use std::fs::File;

fn main() -> Result<(), String> {
    let input_file_path = std::env::args()
        .nth(1)
        .ok_or_else(|| "USAGE: merchants <input_file_path>".to_string())?;
    let file = File::open(&input_file_path).map_err(|_err| {
        format!(
            "merchants app could not find the provided input path - '{}'",
            input_file_path
        )
    })?;

    guide::answer(file)
}
