// https://leetcode.com/problems/largest-rectangle-in-histogram/
use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn get_square(mut acc: Vec<(usize, i32)>, idx: usize) -> (i32, Vec<(usize, i32)>) {
        let (mut prev_idx, curr_val) = acc.pop().unwrap();
        //println!("POP Acc len: {} prev_idx: {}", acc.len(), prev_idx);

        for (left_idx, left_val) in acc.iter().rev() {
            if left_val < &curr_val {
                break;
            }
            prev_idx = *left_idx;
        }

        let resp = (idx - prev_idx) as i32 * curr_val;
        //println!("FOR: curr val={} got max={} prev_idx={} idx={}", curr_val, resp, prev_idx, idx);
        return (resp, acc);
    }

    #[inline(always)]
    pub fn step(mut acc: Vec<(usize, i32)>, idx: usize, value: i32, mut max_square: i32) ->
        (i32, Vec<(usize, i32)>) {
        if acc.len() == 0 {
                //println!("Push: idx={} value={}", idx, *value);
            acc.push((idx, value));
            return (max_square, acc);
            }

            let mut proc_idx = idx;
            let mut proc_value = value;

            while acc.last() != None {
                let rr = acc.last().unwrap();
                proc_value = rr.1;
                let proc_idx_c = rr.0;

                if proc_value > value {
                    let r = Solution::get_square(acc, idx);
                    acc = r.1;
                    let curr_max = r.0;
                    max_square = cmp::max(curr_max, max_square);
                    //println!("max square: {} curr_max: {}", max_square, curr_max);

                    proc_idx = proc_idx_c;
                } else {
                    break;
                }
            }
            //println!("Push PREV: idx={} value={}", proc_idx, *value);
            //println!("value={} proc_value={}", value, proc_value);
            if value != proc_value {
                acc.push((proc_idx, value));
            }
            return (max_square, acc);
    }

    pub fn largest_rectangle_area(nums: Vec<i32>) -> i32 {
        let mut acc: Vec<(usize, i32)> = Vec::new();
        let mut max_square: i32 = 0;
        let mut last_idx = 0;

        for (idx, value) in nums.iter().enumerate() {
            last_idx = idx;
            let r = Solution::step(acc, idx, *value, max_square);
            max_square = r.0;
            acc = r.1;
        }
        let r = Solution::step(acc, last_idx+1, 0, max_square);
        max_square = r.0;
        acc = r.1;
        return max_square;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_84() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![1, 1, 1, 1, 1, 1]), 6);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 2]), 3);
        assert_eq!(Solution::largest_rectangle_area(vec![4, 3, 4, 3, 2, 1, 3]), 12);
        assert_eq!(Solution::largest_rectangle_area(vec![8, 8, 8, 8, 8, 8, 8, 8]), 64);
        assert_eq!(Solution::largest_rectangle_area(vec![8035; 67419]), 541711665);
    }
}
