/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = 1;
        let mut i = digits.len() - 1;
        while i >= 0 {
            let bit = (digits[i] + carry) % 10;
            carry = (digits[i] + carry) / 10;
            digits[i] = bit;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        if carry > 0 {
            digits.insert(0, 1);
        }
        digits
    }
}
// @lc code=end
