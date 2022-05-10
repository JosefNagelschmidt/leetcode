use std::iter::successors;

fn evaluate(x: i32, n_digits: usize, odd: bool) -> bool {
    if n_digits == 1 {
        return true;
    }
    let mut rest = x.clone();
    let mut tmp: Vec<i32> = vec![];
    let mut divisor: i32 = 10_i32.pow((n_digits - 1) as u32);
    for i in 0..n_digits {
        if i == 0 {
            tmp.push(rest / divisor);
            rest = rest % divisor;
            divisor = divisor / 10;
            continue;
        }
        if odd && (i == (n_digits / 2)) {
            rest = rest % divisor;
            divisor = divisor / 10;
            continue;
        }
        if (tmp.iter().last() == Some(&(rest / divisor))) && i >= (n_digits / 2) {
            tmp.pop();
            rest = rest % divisor;
            divisor = divisor / 10;
            continue;
        } else {
            tmp.push(rest / divisor);
            rest = rest % divisor;
            divisor = divisor / 10;
            continue;
        }
    }
    tmp.is_empty()
}

fn main() {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let n_digits = successors(Some(x), |&x| (x >= 10).then(|| x / 10)).count();
        let odd = n_digits % 2;
        let odd = odd == 1;
        evaluate(x, n_digits, odd)
    }
    // println!("{:?}", is_palindrome(121));
}
