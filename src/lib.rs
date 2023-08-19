use clap::Parser;
use regex::Regex;
use std::error::Error;
use std::fs::{read_to_string, write};

/// Structure for command line arguments
#[derive(Parser, Debug)]
#[command(author = "m62624")]
#[command(version = "0.2.1")]
#[command(about = "Retrieve the fragment from the file")]
pub struct Fragment {
    /// read from file
    #[arg(short, long)]
    read_from: String,
    /// start string (Regular Expression is available)
    #[arg(short, long)]
    start_str: String,
    /// end string (Optional, if not specified || no final match is found, the file is read to the end. Regular Expression is available)
    #[arg(short, long)]
    end_str: Option<String>,
    /// write to file (Optional, if not specified, output to stdout)
    #[arg(short, long)]
    write_to: Option<String>,
}

/// Run the program
pub fn runner() -> Result<(), Box<dyn Error>> {
    let cli = Fragment::parse();

    // Read from file
    let all_content = read_to_string(&cli.read_from)?;

    // Start string
    let re = Regex::new(&cli.start_str)?;
    let mut fragment = re.captures_iter(&all_content);

    // Get start index if available
    let start_index = fragment
        .next()
        .ok_or("No match found for start string")?
        .get(0)
        .ok_or("No capture found for start string")?
        .start();

    // Get end index if available
    let fragment = if let Some(end_index_str) = &cli.end_str {
        let re_end = Regex::new(&end_index_str)?;
        let end_index = re_end
            .find(&all_content)
            .map(|m| m.start())
            .unwrap_or_else(|| all_content.len());
        &all_content[start_index..end_index]
    } else if let Some(end_index) = fragment.next() {
        let end_index = end_index
            .get(0)
            .ok_or("No capture found for end string")?
            .start();
        &all_content[start_index..end_index]
    } else {
        &all_content[start_index..]
    };

    // Write to file or stdout
    if let Some(write_to) = &cli.write_to {
        write(write_to, fragment)?;
    } else {
        println!("{}", fragment);
    }

    Ok(())
}
