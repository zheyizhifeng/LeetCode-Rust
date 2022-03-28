/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut ans = 0;
        let mut new_x = x;
        loop {
            if new_x != 0 {
                if ans > i32::MAX / 10 || ans < i32::MIN / 10 {
                    return 0;
                }
                ans = ans * 10 + new_x % 10;
                new_x = new_x / 10;
            } else {
                break;
            }
        }
        ans
    }
}
// @lc code=end
