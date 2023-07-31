pub enum File {}

impl File {
    pub fn access_file(file_path: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(file_path)
    }

    pub fn get_params(n: usize) -> Option<String> {
        std::env::args().nth(n)
    }
}

macro_rules! get_target_file_content {
    () => {
        {
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

            file_content
        }
    };
}
