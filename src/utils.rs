pub fn parse_args(s: &str) -> Vec<String> {
    let mut args = Vec::new();

    let mut current = String::new();
    let mut within_single_quote = false;
    let mut within_double_quote = false;
    let mut has_content = false;

    for ch in s.chars() {
        match ch {
            '\"' if !within_single_quote => {
                within_double_quote = !within_double_quote;
                has_content = true;
            }
            '\'' if !within_double_quote => {
                within_single_quote = !within_single_quote;
                has_content = true;
            }
            ' ' if !within_single_quote && !within_double_quote => {
                if has_content {
                    args.push(current.clone());
                    current.clear();
                    has_content = false;
                }
            }
            _ => {
                current.push(ch);
                has_content = true;
            }
        }
    }

    if has_content {
        args.push(current);
    }

    args
}

