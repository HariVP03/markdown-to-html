use crate::utils::File;
#[macro_use]
pub mod utils;
fn main() {
    let file_name: String = match File::get_params(1) {
        Some(file) => file,
        None => {
            println!("Please provide a file name");
            return;
        }
    };

    let file_content = match File::access_file(file_name) {
        Ok(content) => content,
        Err(error) => {
            println!("Error: {}", error.to_string());
            return;
        }
    };

    println!("{}", file_content);
}
