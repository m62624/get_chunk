use clap::Parser;
use regex::Regex;
use std::fs::read_to_string;
use std::fs::write;

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

pub fn runner() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Fragment::parse();
    let all_content = read_to_string(cli.read_from)?;
    let re = Regex::new(&cli.start_str)?;
    let mut fragment = re.captures_iter(&all_content);
    let fragment = if let Some(end_index) = cli.end_str {
        let re_end = Regex::new(&end_index)?;
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
    if let Some(write_to) = cli.write_to {
        write(write_to, fragment)?;
    } else {
        println!("{}", fragment);
    }
    //     &all_content[fragment.next().unwrap().get(0).unwrap().start()..frg_end.start()]
    // } else {
    //     &all_content[fragment.next().unwrap().get(0).unwrap().start()..]
    // };
    // let re_end = Regex::new(&end_str)?;
    // let mut fragment_end = re_end.find(&all_content);
    // let fragment = if let Some(frg_end) = fragment_end

    Ok(())
}
