use clap::Parser;
use regex::Regex;
use std::fs::read_to_string;
use std::fs::write;

/// Structure for command line arguments
#[derive(Parser, Debug)]
#[command(author = "m62624")]
#[command(version = "1.0.0")]
#[command(about = "Retrieve the fragment from the file")]
pub struct Fragment {
    /// read from file
    #[arg(short, long)]
    read_from: String,
    /// start string (Regular Expression is available)
    #[arg(short, long)]
    start_str: String,
    /// end string (Regular Expression is available)
    #[arg(short, long)]
    end_str: Option<String>,
    /// write to file (optional, if not specified, output to stdout)
    #[arg(short, long)]
    write_to: Option<String>,
}

/// Run the program
pub fn runner() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Fragment::parse();
    // Read from file
    let all_content = read_to_string(cli.read_from)?;
    // Start string
    let re = Regex::new(&cli.start_str)?;
    // Captures for start index
    let mut fragment = re.captures_iter(&all_content);
    // Check if there is a template for the final index
    let fragment = if let Some(end_index) = cli.end_str {
        // End string
        let re_end = Regex::new(&end_index)?;
        // Check if there is a template for the final index
        if let Some(end_index) = re_end.find(&all_content) {
            &all_content[fragment.next().unwrap().get(0).unwrap().start()..end_index.start()]
        } else {
            &all_content[fragment.next().unwrap().get(0).unwrap().start()..]
        }
    } else {
        let start_index = fragment.next().unwrap().get(0).unwrap().start();
        if let Some(end_index) = fragment.next() {
            &all_content[start_index..end_index.get(0).unwrap().start()]
        } else {
            &all_content[start_index..]
        }
    };
    // Write to file or stdout
    if let Some(write_to) = cli.write_to {
        write(write_to, fragment)?;
    } else {
        println!("{}", fragment);
    }
    Ok(())
}
