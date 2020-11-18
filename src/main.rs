extern crate markdown;

use std::{env, fs, path::Path};
use regex::Regex;

enum ReadError {
    IO(String),
    FileNotFound
}

fn read_from_file(path: String) -> Result<String, ReadError> {
    let file_path = Path::new(&path);

    let output = match file_path.is_file() {
        true => match fs::read_to_string(file_path) {
            Ok(text) => text,
            Err(error) => return Err(ReadError::IO(error.to_string())),
        },
        false => return Err(ReadError::FileNotFound)
    };

    Ok(output)
}

enum FillContentError {
    RegexFailed
}

fn fill_content(html: String, sections_directory: String) -> Result<String, FillContentError> {
    let search = match Regex::new(r"\{\{[[:alnum:]]+\}\}") {
        Ok(regex) => regex,
        Err(_) => return Err(FillContentError::RegexFailed)
    };

    let matches = search.find_iter(&html);

    let filled_content = matches.fold(html.clone(), |result, m| {
        let content_name = m.as_str().replace("{", "").replace("}", "");

        let next = match read_from_file(format!("{}/{}.md", sections_directory, content_name)) {
            Ok(text) => {
                let parsed_markdown = markdown::to_html(&text);
                result.replace(m.as_str(), &parsed_markdown)
            }
            Err(_) => result.clone()
        };

        next
    });

    Ok(filled_content)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No resume path specified.");
        return;
    }

    let file_name = args[1].clone();

    let text = match read_from_file(file_name.clone()) {
        Ok(out) => out,
        Err(error) => {
            let error_message = match error {
                ReadError::IO(message) => format!("IO Error while reading file '{}': {}.", file_name, message),
                ReadError::FileNotFound => format!("File '{}' does not exist.", file_name)
            };

            println!("{}", error_message);
            return;
        }
    };

    if args.len() < 3 {
        println!("No sections directory specified.");
        return;
    }

    let result = match fill_content(text, args[2].clone()) {
        Ok(r) => r,
        Err(err) => {
            let error_message = match err {
                FillContentError::RegexFailed => "Failed to build search regex."
            };

            println!("{}", error_message);
            return;
        }
    };

    if args.len() < 4 {
        println!("No output specified.");
        return;
    }

    match fs::write(args[3].clone(), result) {
        Err(_) => println!("Failed to write output."),
        _ => return
    }
}
