// https://leetcode.com/problems/rotate-array/
pub struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        //let norm_k = k as usize % nums.len();
        //nums.rotate_right(norm_k);

        let norm_k = (k as usize) % nums.len();
        nums.reverse();
        nums[norm_k..].reverse();
        nums[..norm_k].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(mut v1: Vec<i32>, k: i32, v2: Vec<i32>) {
        Solution::rotate(&mut v1, k);
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_189() {
        // let mut v = vec![1, 2, 3, 4];
        // Solution::rotate(&mut v, 2);
        // assert_eq!(v, vec![3, 4, 1, 2]);
        t(vec![1, 2, 3, 4], 2, vec![3, 4, 1, 2]);
        t(vec![1, 2, 3, 4], 3, vec![2, 3, 4, 1]);
    }
}
