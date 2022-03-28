/*
 * @lc app=leetcode.cn id=13 lang=rust
 *
 * [13] 罗马数字转整数
 */

// @lc code=start
// struct Solution {}
impl Solution {
    fn get_roman_int(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
    pub fn roman_to_int(s: String) -> i32 {
        let mut ans = 0;
        s.chars().rev().fold(0, |prev_int, cur| {
            let cur_int = Solution::get_roman_int(cur);
            if cur_int < prev_int {
                ans -= cur_int;
            } else {
                ans += cur_int;
            }
            return cur_int;
        });
        return ans;
    }
}
// fn main() {
//     Solution::roman_to_int("MCMXCIV".to_string());
// }
// @lc code=end
