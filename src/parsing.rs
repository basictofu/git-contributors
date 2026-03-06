use regex::Regex;

pub fn parse_line(line: &str) -> Option<(u32, String)> {
    let splits: Vec<&str> = line.split(" ").collect();
    let first = splits[0];
    let num: u32;
    match first.parse::<u32>() {
        Ok(number) => {
            num = number;
        }
        Err(_) => {
            return None;
        }
    }
    let rest = splits[1..].join(" ");
    let Ok(author) = parse_author(&rest) else { return None };
    return Some((num, author));
}

fn parse_author(s: &str) -> Result<String, &str> {
    let re = Regex::new(r"([^<]*)(<.*>)?").unwrap();
    let Some(caps) = re.captures(s) else { return Err("Could not parse author"); };
    let author = caps[1].trim().to_string();
    
    return Ok(author);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let parsed = parse_line("123 bob").unwrap();
        assert_eq!(parsed, (123 as u32, format!("bob")));
    }

    #[test]
    fn test_failed_parse_line() {
        let not_parsed = parse_line("hey bob");
        assert_eq!(not_parsed, None);
    }
    
    #[test]
    fn test_parse_author() {
        let author = parse_author("bob <email>").expect("Could not parse author");
        assert_eq!(author, "bob");
    }
}
