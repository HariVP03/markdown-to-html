pub enum Parser {}

impl Parser {
    fn parse_headings(string: String, html_content: &mut String) {
        string.split("\n").for_each(|line: &str| {
            if line.starts_with("###") {
                *html_content += format!("<h3>{}</h3>", &line[3..]).as_str();
            } else if line.starts_with("##") {
                *html_content += format!("<h2>{}</h2>", &line[2..]).as_str();
            } else if line.starts_with("#") {
                *html_content += format!("<h1>{}</h1>", &line[1..]).as_str();
            }
        })
    }

    fn parse_wrapper(
        string: String,
        markdown_wrapper: String,
        html_tag: String,
        html_content: &mut String
    ) -> Result<String, String> {
        let mut index: usize = 0;
        let mut start_of_wrapper_temp: i32 = 0;
        let mut completion_flag: bool = true;

        while index < string.len() {
            if string[index..markdown_wrapper.len()] == markdown_wrapper {
                start_of_wrapper_temp = index as i32;

                completion_flag = false;
                *html_content += format!("<{}>", html_tag).as_str();

                while index < string.len() {
                    if string[index..markdown_wrapper.len()] == markdown_wrapper {
                        *html_content += format!("</{}>", html_tag).as_str();
                        index += markdown_wrapper.len();
                        completion_flag = true;
                        break;
                    } else {
                        *html_content += string[index..index + 1].to_string().as_str();
                        index += 1;
                    }
                }
            }
        }

        if !completion_flag {
            println!(
                "The markdown you provided does not seem to be valid. Please check your markdown at:"
            );
            let context: String =
                string[start_of_wrapper_temp as usize..markdown_wrapper.len() + 5].to_string();

            Err(format!("{}\n{}", context, "^".repeat(context.len())))
        } else {
            Ok(html_content.to_string())
        }
    }

    fn parse(string: String) {
        let mut html_content: String = "".to_string();

        // Parser::parse_headings(string, &mut html_content);
    }
}
