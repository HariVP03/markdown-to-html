use crate::file::File;

#[macro_use]
pub mod file;
pub mod parser;

fn main() {
    let file_content: String = get_target_file_content!();

    println!("{}", &file_content);
}
