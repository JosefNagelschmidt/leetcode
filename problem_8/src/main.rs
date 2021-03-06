fn main() {
    pub fn my_atoi(string: String) -> i32 {
        let mut chrs = string.chars().skip_while(|c| c == &' ').peekable();

        let sign = if chrs.peek().map_or(false, |s| s == &'-') {
            chrs.next();
            -1i32
        } else {
            if chrs.peek().map_or(false, |s| s == &'+') {
                chrs.next();
            }

            1i32
        };

        chrs.into_iter()
            .take_while(|n| n.is_numeric())
            .try_fold(0i32, |acc, n| {
                acc.checked_mul(10)
                    .and_then(|acc| acc.checked_add(n.to_digit(10).unwrap() as i32))
            })
            .map(|n| n * sign)
            .unwrap_or(if sign > 0 {
                std::i32::MAX
            } else {
                std::i32::MIN
            })
    }
    // println!("{:?}", my_atoi("   -789   ".to_string()));
}
