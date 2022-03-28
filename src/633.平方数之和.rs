/*
 * @lc app=leetcode.cn id=633 lang=rust
 *
 * [633] 平方数之和
 */

// @lc code=start
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut left = 0;
        let mut right = (c as f64).sqrt() as i32;
        while left <= right {
            let diff = c - right * right;
            if diff == left * left {
                return true;
            } else if diff > left * left {
                left += 1;
            } else {
                right -= 1;
            }
        }
        return false;
    }
}
// @lc code=end
