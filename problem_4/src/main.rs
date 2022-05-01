use std::cmp;

fn main() {
    fn get_val(arr: &Vec<i32>, index: i32) -> i32 {
        if index == -1 {
            return i32::MIN;
        } else if index == arr.len() as i32 {
            return i32::MAX;
        } else {
            return arr[index as usize];
        }
    }
    // return (l_short, r_short, l_long, r_long)
    fn candidate_indeces(
        r_short: i32,
        a_short: &Vec<i32>,
        a_long: &Vec<i32>,
    ) -> (i32, i32, i32, i32) {
        let mid_index = (a_short.len() + a_long.len()) as i32 / 2;
        let r_long = mid_index - r_short;
        (r_short - 1, r_short, r_long - 1, r_long)
    }

    fn binary_search_direction(
        l_short: i32,
        r_short: i32,
        l_long: i32,
        r_long: i32,
        a_short: &Vec<i32>,
        a_long: &Vec<i32>,
    ) -> i32 {
        if get_val(&a_short, l_short) > get_val(&a_long, r_long) {
            return -1;
        } else if get_val(&a_long, l_long) > get_val(&a_short, r_short) {
            return 1;
        } else {
            return 0;
        }
    }

    fn get_median(
        l_short: i32,
        r_short: i32,
        l_long: i32,
        r_long: i32,
        a_short: &Vec<i32>,
        a_long: &Vec<i32>,
    ) -> f64 {
        let odd = (a_short.len() + a_long.len()) % 2;
        let odd = if odd == 1 { true } else { false };
        if odd {
            return cmp::min(get_val(&a_long, r_long), get_val(&a_short, r_short)) as f64;
        } else {
            return (cmp::max(get_val(&a_short, l_short), get_val(a_long, l_long))
                + cmp::min(get_val(&a_short, r_short), get_val(a_long, r_long)))
                as f64
                / 2.0;
        }
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let a_short: Vec<i32>;
        let a_long: Vec<i32>;
        let mut direction: i32 = 1;
        let mut split_point: i32 = 1;
        let mut l_short: i32 = 1;
        let mut r_short: i32 = 1;
        let mut l_long: i32 = 1;
        let mut r_long: i32 = 1;

        if nums1.len() < nums2.len() {
            a_short = nums1;
            a_long = nums2;
        } else {
            a_short = nums2;
            a_long = nums1;
        }

        let mut l: i32 = 0;
        let mut r: i32 = a_short.len() as i32;

        while direction != 0 {
            split_point = (l + r) / 2;

            let index_tuple = candidate_indeces(split_point, &a_short, &a_long);

            l_short = index_tuple.0;
            r_short = index_tuple.1;
            l_long = index_tuple.2;
            r_long = index_tuple.3;

            direction =
                binary_search_direction(l_short, r_short, l_long, r_long, &a_short, &a_long);

            if direction < 0 {
                r = split_point - 1;
            } else if direction > 0 {
                l = split_point + 1;
            }
        }

        return get_median(l_short, r_short, l_long, r_long, &a_short, &a_long);
    }

    // let med = find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5, 6]);
    // println!("{:?}", med);
}
