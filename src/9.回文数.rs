/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            false
        } else {
            let mut left = x;
            let mut right = 0;
            loop {
                if (left > right) {
                    right = right * 10 + left % 10;
                    left = left / 10;
                } else {
                    break;
                }
            }
            left == right || left == right / 10
        }
    }
}
// @lc code=end
