fn main() {
    fn longest_palindrome(s: String) -> String {
        let input_length = s.len();

        if input_length == 0 {
            return String::from("");
        }

        let mut flags = vec![vec![false; input_length]; input_length];

        let mut res_start_ind = 0;
        let mut res_letters = 1;
        let b = s.as_bytes();

        for end in 0..input_length {
            flags[end][end] = true;

            for start in 0..end {
                flags[start][end] =
                    b[start] == b[end] && (end - start < 3 || flags[start + 1][end - 1]);
                if flags[start][end] && end - start + 1 > res_letters {
                    res_start_ind = start;
                    res_letters = end - start + 1;
                }
            }
        }

        let res_end_ind = res_start_ind + res_letters;
        return (s[res_start_ind as usize..res_end_ind as usize]).to_string();
    }
    // let a = longest_palindrome(String::from("adam"));
    // println!("{:?}", a);
}
