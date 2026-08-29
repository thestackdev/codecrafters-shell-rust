enum Priority {
    SingleQuote(u8),
    Space(u8),
}

fn get_priority(ch: &char) -> u8 {
    match ch {
        '\'' => 2,
        ' ' => 1,
        _ => 0,
    }
}

pub fn parse_args(s: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut in_quote = false;
    let mut current = String::new();
    let mut has_content = false;

    for ch in s.chars() {
        match ch {
            '\'' => {
                in_quote = !in_quote;
                has_content = true;
            }
            ' ' if !in_quote => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        let s = String::from("hello 'test'");
        assert_eq!(
            parse_args(&s),
            vec![String::from("hello"), String::from("test")]
        );
    }
}
