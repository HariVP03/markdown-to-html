use crate::file::File;
use crate::parser::Parser;

#[macro_use]
pub mod file;
pub mod parser;

fn main() {
    let (file_content, file_path) = get_target_file_content!();
    let parsed_html: String = Parser::parse(&file_content);

    match File::write_file(&file_path, parsed_html) {
        Ok(_) => println!("Successfully converted markdown to html"),
        Err(error) => println!("Error: {}", error.to_string()),
    }
}
