pub enum File {}

impl File {
    pub fn access_file(file_path: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(file_path)
    }

    pub fn get_params(n: usize) -> Option<String> {
        std::env::args().nth(n)
    }
}
