fn convert_thousands(num: i32) -> Option<String> {
    let thousand_digit = num / 1000;
    if thousand_digit > 0 {
        let v = vec!['M'; thousand_digit as usize];
        return Some(v.into_iter().collect());
    } else {
        return None;
    }
}
fn convert_hundreds(num: i32) -> Option<String> {
    let hundred_digit = num / 100;
    if hundred_digit > 0 {
        let hundred_digit = match hundred_digit
            .to_string()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
        {
            Some(x) => x as i32,
            None => {
                panic!("Error")
            }
        };
        let mut v: Vec<char>;
        if hundred_digit == 9 {
            return Some("CM".to_string());
        } else if hundred_digit > 5 {
            let ones = hundred_digit - 5;
            v = vec!['D'];
            let v2 = vec!['C'; ones as usize];
            v.extend(v2);
            return Some(v.into_iter().collect());
        } else if hundred_digit == 5 {
            return Some("D".to_string());
        } else if hundred_digit == 4 {
            return Some("CD".to_string());
        } else if hundred_digit < 4 {
            return Some(vec!['C'; hundred_digit as usize].into_iter().collect());
        } else {
            panic!("Error")
        }
    } else {
        return None;
    }
}
fn convert_tens(num: i32) -> Option<String> {
    let tens_digit = num / 10;
    if tens_digit > 0 {
        let tens_digit = match tens_digit.to_string().chars().last().unwrap().to_digit(10) {
            Some(x) => x as i32,
            None => {
                panic!("Error")
            }
        };
        let mut v: Vec<char>;
        if tens_digit == 9 {
            return Some("XC".to_string());
        } else if tens_digit > 5 {
            let ones = tens_digit - 5;
            v = vec!['L'];
            let v2 = vec!['X'; ones as usize];
            v.extend(v2);
            return Some(v.into_iter().collect());
        } else if tens_digit == 5 {
            return Some("L".to_string());
        } else if tens_digit == 4 {
            return Some("XL".to_string());
        } else if tens_digit < 4 {
            return Some(vec!['X'; tens_digit as usize].into_iter().collect());
        } else {
            panic!("Error");
        }
    } else {
        return None;
    }
}
fn convert_single_digits(num: i32) -> String {
    let single_digit = match num.to_string().chars().last().unwrap().to_digit(10) {
        Some(x) => x as i32,
        None => {
            panic!("Error")
        }
    };
    let mut v: Vec<char>;
    if single_digit == 9 {
        return "IX".to_string();
    } else if single_digit > 5 {
        let ones = single_digit - 5;
        v = vec!['V'];
        let v2 = vec!['I'; ones as usize];
        v.extend(v2);
        return v.into_iter().collect();
    } else if single_digit == 5 {
        return "V".to_string();
    } else if single_digit == 4 {
        return "IV".to_string();
    } else if single_digit < 4 {
        return vec!['I'; single_digit as usize].into_iter().collect();
    } else {
        return "".to_string();
    }
}

fn main() {
    pub fn int_to_roman(num: i32) -> String {
        let mut res: Vec<String> = vec![];

        let thousand_digit = convert_thousands(num);
        if let Some(x) = thousand_digit {
            res.push(x);
        }

        let hundred_digit = convert_hundreds(num);
        if let Some(x) = hundred_digit {
            res.push(x);
        }

        let tens_digit = convert_tens(num);
        if let Some(x) = tens_digit {
            res.push(x);
        }

        let tens_digit = convert_single_digits(num);
        res.push(tens_digit);

        return res.join("");
    }
    // println!("{}", int_to_roman(1999));
}
