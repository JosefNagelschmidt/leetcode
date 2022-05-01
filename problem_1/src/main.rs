use std::collections::HashMap;
fn main() {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, digit) in nums.iter().enumerate() {
            let diff = target - digit;
            let remainder_index = match map.get(&diff) {
                None => {
                    map.insert(*digit, i as i32);
                    continue;
                }
                Some(x) => *x,
            };
            return vec![i as i32, remainder_index];
        }
        return vec![];
    }
    // let res = two_sum(vec![2, 7, 11, 15], 9);
    // println!("{:?}", res);
}
