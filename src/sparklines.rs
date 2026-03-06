use std::vec::Vec;

fn bar(i: u8) -> String {
    return match i {
        0 => " ".to_string(),
        1 => "▁".to_string(),
        2 => "▂".to_string(),
        3 => "▃".to_string(),
        4 => "▄".to_string(),
        5 => "▅".to_string(),
        6 => "▆".to_string(),
        7 => "▇".to_string(),
        8 => "█".to_string(),
        _ => " ".to_string(),
    };
}

pub fn spark(vals: Vec<u8>) -> String {
    return vals
        .iter()
        .map(|&i| bar(i))
        .collect::<Vec<String>>()
        .join("");
}

#[test]
fn test_spark() {
    let vals: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let s = spark(vals);
    assert_eq!(s, "▁▂▃▄▅▆▇█");
}
