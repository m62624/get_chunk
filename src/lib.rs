use regex::Regex;
use std::env::args_os;
use std::fs::read_to_string;

#[derive(Default)]
pub struct Fragment {
    start_str: Option<String>,
    end_str: Option<String>,
    path_from_read: Option<String>,
    path_to_write: Option<String>,
    mode: ResultType,
}
pub enum ResultType {
    ShowInfo,
    WriteInfo,
}

impl Default for ResultType {
    fn default() -> Self {
        ResultType::ShowInfo
    }
}

impl Fragment {
    fn is_some(&self) -> bool {
        self.start_str.is_some() && self.path_from_read.is_some()
    }
}

pub fn get_parametrs() -> Vec<String> {
    let mut args = args_os();
    args.next();
    let mut params = Vec::new();
    for arg in args {
        params.push(arg.into_string().unwrap_or_else(|arg| {
            eprintln!("Error: Argument '{:?}' is not valid UTF-8.", arg);
            std::process::exit(1);
        }));
    }
    params
}

pub fn check_and_write_options(params: Vec<String>) -> Fragment {
    let mut fragment = Fragment::default();
    if !params.len() == 0 {
        for param in params.clone() {
            match param.as_str() {
                "-h" | "--help" => {
                    println!("Program to retrieve a text fragment starting from `s | start_str` to `e | end_str` from a `pr | path_from_read` file, and output the result to the console or write to a file.\n`s | start_str` - Regular expressions are supported\n`e | end_str` - Regular expressions are supported\n`p | path_from_read` - from where to get the data fragment, the path to the file\n`pw | path_to_write` - where to write the result, the path to the file
                    \n`sr | --show-result` - Show result\n`wr | --write-result` - Write result to file\n`h | --help` - Help\n\nFor regular expressions, the https://crates.io/crates/regex crate is used. ");
                }
                "-s" | "--start_str" => {
                    fragment.start_str = Some(param);
                }
                "-e" | "--end_str" => {
                    fragment.end_str = Some(param);
                }
                "-pr" | "--path_from_read" => {
                    fragment.path_from_read = Some(param);
                }
                "-pw" | "--path_to_write" => {
                    fragment.path_to_write = Some(param);
                }
                "-sr" | "--show-result" => {
                    if params.contains(&"-wr".into()) || params.contains(&"--write-result".into()) {
                        eprintln!("Error parameters [-sr | --show-result] and [-wr | --write-result] cannot be used together")
                    }
                    fragment.mode = ResultType::ShowInfo;
                }
                "-wr" | "--write-result" => {
                    if params.contains(&"-sr".into()) || params.contains(&"--show-result".into()) {
                        eprintln!("Error parameters [-sr | --show-result] and [-wr | --write-result] cannot be used together")
                    }
                    fragment.mode = ResultType::WriteInfo;
                }
                _ => {
                    println!("No parameters [-h | --help]");
                }
            }
        }
    } else {
        println!("No parameters");
    }
    fragment
}

pub fn runner(data: Fragment) {
    if data.is_some() {
        let all_content = read_to_string(data.path_from_read.unwrap()).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        });
        // let re = Regex::new(format!("{},{}",data.)).unwrap_or_else(|err| {
        //     eprintln!("Error: {}", err);
        //     std::process::exit(1);
        // });
        // let fragment  = re.captures_iter(&all_content);
        // let fragment = all_content[];
    } else {
        println!("All parameters must be specified (more information -h | --help)");
    }
}
