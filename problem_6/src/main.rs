use std::collections::BTreeMap;

fn push_to_hashmap(k: usize, c: char, map: &mut BTreeMap<usize, String>) {
    if let Some(x) = map.get_mut(&k) {
        x.push(c);
    } else {
        map.insert(k, c.to_string());
    }
}

fn main() {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut map = BTreeMap::new();
        let mut row: usize = 0;
        let mut zigzag_move: bool = false;

        for c in s.chars() {
            if zigzag_move {
                row = row - 1;
                push_to_hashmap(row, c, &mut map);
                if row == 0 {
                    zigzag_move = false;
                    row = row + 1;
                }
            } else {
                push_to_hashmap(row, c, &mut map);
                row = row + 1;
                if row == num_rows as usize {
                    zigzag_move = true;
                    row = row - 1;
                }
            }
        }
        let values: Vec<String> = map.values().cloned().collect();
        return values.join("");
    }
    // let res = convert("PAYPALISHIRING".to_string(), 4);
    // println!("{:?}", res);
}
