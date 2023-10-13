use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::fs::OpenOptions;


fn main() -> io::Result<()> {
    // Open the input file
    let input_file = File::open("src/lib/hello_wat.txt")?;
    let input_file = io::BufReader::new(input_file);

    // Open the output file
    let mut output_file = File::create("output.rs")?;

    let file_path = "output.rs";
    // Open the existing file in write mode with OpenOptions
    let mut output_file = OpenOptions::new()
        .write(true) // Open in write mode
        .append(true) // Append data to the file (optional)
        .open(file_path)?;

    // Read lines from the input file and process them
    for line in input_file.lines() {
        let line = line?; // Unwrap the line

        // Perform a match on the line content
        let words: Vec<&str> = line.split_whitespace().collect();
        let keyword = words[1];
        println!("{}",keyword);


        match keyword.trim() {
            "Import" => {
                // Case "case0": Write to the output file
                
                writeln!(output_file,"This is case 0\n")?;
            },
            "Export" => {
                // Cases "case1" and "case2": Write to the output file
                
                writeln!(output_file, "This is case 1\n")?;
            },
            "I32Sub" => {
                // Cases "case3" to "case10": Write to the output file
                
                writeln!(output_file, "This is case 2\n")?;
            },
            _ => {
                // Default case: Write to the output file
                writeln!(output_file, "This is case 3\n")?;
            },
        }
    }

    Ok(())
}
