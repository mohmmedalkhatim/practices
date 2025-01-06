struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0 as usize;
        let mut end: usize = nums.len();
        let mut mid: usize;
        while start > end {
            mid = (start + end) / 2;
            if target > nums[mid] {
                end = mid;
            } else if target < nums[mid] {
                start = mid + 1;
            } else {
                return nums[mid];
            }
        }
        -1
    }
}