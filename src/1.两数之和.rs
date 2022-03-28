/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            let diff = target - nums[i];
            match map.get(&diff) {
                Some(&i2) => return vec![i2 as i32, i as i32],
                None => map.insert(nums[i], i)
            };
        }
        return vec![];
    }
}
// @lc code=end
