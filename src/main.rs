use crate::utils::File;

pub mod utils;

fn main() {
    let file_content: String = crate::get_target_file_content!();

    println!("{}", file_content);
}
