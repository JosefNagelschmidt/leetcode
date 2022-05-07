// use std::cmp;

// fn reduce(height: &Vec<i32>, remaining_indeces: &mut Vec<usize>) -> bool {
//     let n_ante = remaining_indeces.len();
//     if n_ante < 3 {
//         true;
//     }
//     let mut keep = vec![true; n_ante];

//     let triple_iter = remaining_indeces.windows(3);
//     triple_iter.enumerate().for_each(|(i, triple)| {
//         if (height[triple[0]] >= height[triple[1]]) & (height[triple[2]] >= height[triple[1]]) {
//             keep[i + 1] = false;
//         }
//     });
//     let mut iter = keep.iter();
//     remaining_indeces.retain(|_| *iter.next().unwrap());

//     if n_ante == remaining_indeces.len() {
//         true
//     } else {
//         false
//     }
// }

// fn get_max_area(height: &Vec<i32>, remaining_indeces: &Vec<usize>) -> i32 {
//     let remaining_heights = remaining_indeces
//         .iter()
//         .map(|i| height[*i])
//         .collect::<Vec<_>>();
//     let mut maximum = -1;
//     let mut val: i32;
//     for left in 0..remaining_indeces.len() - 1 {
//         for right in left + 1..remaining_indeces.len() {
//             val = (remaining_indeces[right] as i32 - remaining_indeces[left] as i32)
//                 * (cmp::min(remaining_heights[left], remaining_heights[right]));
//             if val > maximum {
//                 maximum = val;
//             }
//         }
//     }
//     return maximum;
// }

fn main() {
    // pub fn max_area(height: Vec<i32>) -> i32 {
    //     let mut reduced: bool = false;
    //     let mut remaining_indeces: Vec<usize> = Vec::from_iter(0..height.len());
    //     while !reduced {
    //         reduced = reduce(&height, &mut remaining_indeces);
    //     }
    //     let max_area = get_max_area(&height, &remaining_indeces);
    //     return max_area;
    // }

    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut maximum: i32 = 0;
        let mut area;
        let mut i: usize = 0;
        let mut j: usize = height.len() - 1;

        while i < j {
            if height[i] <= height[j] {
                area = height[i] * (j - i) as i32;
                i += 1
            } else {
                area = height[j] * (j - i) as i32;
                j -= 1
            }
            if area > maximum {
                maximum = area
            }
        }
        return maximum;
    }

    // let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    // println!("{:?}", max_area(height));
}
