pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    let mut min_distance: i32 = i32::max_value();
    nums.sort();
    let mut i = 0;
    while i < nums.len() - 2 {
        let sub_min = two_sum_closest(&nums[(i + 1)..nums.len()], target - nums[i]);
        if sub_min.abs() < min_distance.abs() {
            min_distance = sub_min;
            if min_distance == 0 {
                break;
            }
        }
        i += 1;
    }
    target + min_distance
}

pub fn two_sum_closest(nums: &[i32], target: i32) -> i32 {
    let (mut i, mut j) = (0_usize, nums.len() - 1);
    let mut local_min = i32::max_value();
    while i < j {
        let sum = nums[i] + nums[j];
        if sum > target {
            j -= 1;
        } else if sum < target {
            i += 1;
        } else {
            return 0;
        }
        if (sum - target).abs() < local_min.abs() {
            local_min = sum - target
        }
    }
    local_min
}

fn main() {
    println!("{:?}", three_sum_closest(vec![-1, 2, 1, -4], -1))
}
