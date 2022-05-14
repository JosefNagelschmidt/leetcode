fn main() {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        match strs.is_empty() {
            true => "".to_string(),
            _ => strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
                acc.chars()
                    .zip(x.chars())
                    .take_while(|(x, y)| x == y)
                    .map(|(x, _)| x)
                    .collect()
            }),
        }
    }
    // println!(
    //     "{:?}",
    //     longest_common_prefix(vec![
    //         "aber".to_string(),
    //         "ab".to_string(),
    //         "abe".to_string(),
    //     ])
    // );
}
