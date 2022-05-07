use std::collections::HashMap;

fn main() {
    pub fn roman_to_int(s: String) -> i32 {
        let subtraction_variants = HashMap::from([
            ("IV", 4),
            ("IX", 9),
            ("XL", 40),
            ("XC", 90),
            ("CD", 400),
            ("CM", 900),
        ]);
        let normal_variants: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let mut num = 0;
        let mut reduced_s = s.clone();

        for (key, val) in subtraction_variants {
            if s.contains(key) {
                num += val;
            }
            reduced_s = reduced_s.replace(key, "");
        }
        for c in reduced_s.chars() {
            let val = normal_variants.get(&c);
            let val = match val {
                Some(x) => x,
                None => panic!("There is an illegal char in the number."),
            };
            num += val;
        }
        return num;
    }
    // println!("{:?}", roman_to_int("MCMXCIV".to_string()));
}
