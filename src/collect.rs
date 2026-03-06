use crate::data::{Collected, RawAuthor};
use crate::parsing::parse_line;
use std::collections::HashMap;
use std::vec::Vec;

pub fn collect_by_author(text: &str) -> Collected {
    let mut min_date = u32::MAX;
    let mut max_date: u32 = 0;

    let mut map: HashMap<String, RawAuthor> = HashMap::new();

    for line in text.lines() {
        match parse_line(line) {
            Some((date, author)) => {
                if date < min_date {
                    min_date = date;
                }
                if date > max_date {
                    max_date = date;
                }
                map.entry(author.to_string())
                    .and_modify(|contributor| {
                        if date < contributor.range.0 {
                            contributor.range.0 = date;
                        }
                        if date > contributor.range.1 {
                            contributor.range.1 = date;
                        }
                        contributor.dates.push(date);
                    })
                    .or_insert(RawAuthor {
                        name: author.to_string(),
                        range: (date, date),
                        dates: vec![date],
                    });
            }
            None => continue,
        }
    }

    let mut ordered_contributors: Vec<RawAuthor> = map.values().map(|c| c.clone()).collect();
    ordered_contributors.sort_by(|a, b| b.dates.len().cmp(&a.dates.len()));

    Collected {
        data_range: (min_date, max_date),
        authors: ordered_contributors,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_collect_simple() {
        let lines = "
1 bob
2 bob
        ";
        let out = collect_by_author(lines);
        let bob = &out.authors[0].dates;
        assert_eq!(bob, &vec![1, 2]);
    }

    #[test]
    fn test_get_total_range() {
        let text = "
0 bob
10 bob
        ";
        let processed = collect_by_author(text);
        let range = processed.data_range;
        assert_eq!(range, (0, 10));
    }

    #[test]
    fn test_first_author_is_top() {
        let text = "
1 alice
2 bob
3 bob
        ";
        let processed = collect_by_author(text);
        let first_author = &processed.authors[0].name;
        assert_eq!(first_author, "bob");
    }
}
