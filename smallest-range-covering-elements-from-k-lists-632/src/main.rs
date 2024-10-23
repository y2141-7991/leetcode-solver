use std::cmp::Reverse;

use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut range, k, mut max) = (None, nums.len(), i32::MIN);
        let mut heap = nums
            .iter()
            .enumerate()
            .fold(BinaryHeap::new(), |mut heap, (idx, vec)| {
                heap.push((Reverse(vec[0]), idx, 0));
                max = max.max(vec[0]);
                heap
            });

        while heap.len() == k {
            if let Some((Reverse(min), list_index, element_index)) = heap.pop() {
                let current_range = max - min;

                match range {
                    None => range = Some((min, max)),
                    Some((start, end)) if current_range < end - start => range = Some((min, max)),
                    Some((start, end)) if current_range == end - start && min < start => {
                        range = Some((min, max))
                    }
                    _ => {}
                }

                if element_index + 1 < nums[list_index].len() {
                    let next = nums[list_index][element_index + 1];
                    heap.push((Reverse(next), list_index, element_index + 1));
                    max = max.max(next);
                } else {
                    break;
                }
            }
        }

        range.map_or_else(Vec::new, |r| vec![r.0, r.1])
    }
}

fn main() {
    let a = vec![
        vec![4, 10, 15, 24, 26],
        vec![0, 9, 12, 20],
        vec![5, 18, 22, 30],
    ];
    println!("{:?}", Solution::smallest_range(a));
}
