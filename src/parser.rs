pub enum Parser {}

impl Parser {
    fn parse_wrapper(
        string: String,
        markdown_wrapper: String,
        html_tag: String,
        html_content: &mut String
    ) {
        let mut index: usize = 0;
        let mut start_of_wrapper_temp: i32 = 0;
        let mut completion_flag: bool = true;

        while index + markdown_wrapper.len() < string.len() {
            if string[index..index + markdown_wrapper.len()] == markdown_wrapper {
                start_of_wrapper_temp = index as i32;

                completion_flag = false;
                *html_content += format!("<{}>", html_tag).as_str();

                index += markdown_wrapper.len();

                while index < string.len() {
                    if index + 2 < string.len() && string[index..index + 2].to_string() == "\n" {
                        continue;
                    }

                    if string[index..index + markdown_wrapper.len()] == markdown_wrapper {
                        *html_content += format!("</{}>", html_tag).as_str();
                        index += markdown_wrapper.len() + 1;
                        completion_flag = true;
                        break;
                    } else {
                        *html_content += string[index..index + 1].to_string().as_str();
                        index += 1;
                    }
                }
            } else {
                index += 1;
            }
        }

        if !completion_flag {
            println!(
                "The markdown you provided does not seem to be valid. Please check your markdown at:"
            );
            let context: String =
                string[start_of_wrapper_temp as usize..markdown_wrapper.len() + 5].to_string();
            println!("{}", context);
        }
    }

    fn parse_headings(string: String, html_content: &mut String) {
        string.split("\n").for_each(|line: &str| {
            if line.starts_with("### ") {
                *html_content += format!("<h3>{}</h3>\n", &line[4..]).as_str();
            } else if line.starts_with("## ") {
                *html_content += format!("<h2>{}</h2>\n", &line[3..]).as_str();
            } else if line.starts_with("# ") {
                *html_content += format!("<h1>{}</h1>\n", &line[2..]).as_str();
            }
        })
    }

    fn parse_bold(string: String, html_content: &mut String) {
        Parser::parse_wrapper(string, "**".to_string(), "b".to_string(), html_content)
    }

    fn parse_italic(string: String, html_content: &mut String) {
        Parser::parse_wrapper(string, "*".to_string(), "i".to_string(), html_content)
    }

    fn parse_underline(string: String, html_content: &mut String) {
        Parser::parse_wrapper(string, "__".to_string(), "u".to_string(), html_content)
    }

    fn parse_strikethrough(string: String, html_content: &mut String) {
        Parser::parse_wrapper(string, "~~".to_string(), "s".to_string(), html_content)
    }

    fn parse_code_block(string: String, html_content: &mut String) {
        Parser::parse_wrapper(string, "```".to_string(), "code".to_string(), html_content)
    }

    pub fn parse(string: &String) -> String {
        let mut html_content: String = "".to_string();

        Parser::parse_headings(string.to_string(), &mut html_content);
        Parser::parse_bold(string.to_string(), &mut html_content);
        Parser::parse_italic(string.to_string(), &mut html_content);
        Parser::parse_underline(string.to_string(), &mut html_content);
        Parser::parse_strikethrough(string.to_string(), &mut html_content);
        Parser::parse_code_block(string.to_string(), &mut html_content);

        html_content
    }
}
