/*
 * @lc app=leetcode.cn id=12 lang=rust
 *
 * [12] 整数转罗马数字
 */

// @lc code=start
struct Solution{}
impl Solution {
    pub fn get_roman_char(num: usize) -> String {
        match num {
            1 => "I".to_string(),
            4 => "IV".to_string(),
            5 => "V".to_string(),
            9 => "IX".to_string(),
            10 => "X".to_string(),
            40 => "XL".to_string(),
            50 => "L".to_string(),
            90 => "XC".to_string(),
            100 => "C".to_string(),
            400 => "CD".to_string(),
            500 => "D".to_string(),
            900 => "CM".to_string(),
            1000 => "M".to_string(),
            _ => "".to_string(),
        }
    }
    pub fn int_to_roman(num: i32) -> String {
        let mut ans = String::new();
        let mut num = num;
        let mut ten_times: usize = 1;
        while num > 0 {
            let bit = (num % 10) as usize;
            if bit == 4 || bit == 9 {
                ans = Solution::get_roman_char(bit * ten_times) + &ans
            } else {
                let five_count = bit / 5;
                let one_count = bit % 5;
                let prev_str = Solution::get_roman_char(five_count * ten_times * 5);
                let next_str = Solution::get_roman_char(ten_times).repeat(one_count);
                ans = prev_str + &next_str + &ans;
            }
            num  = num / 10;
            ten_times *= 10;
        }

        return ans;
    }
}
fn main() {
    Solution::int_to_roman(58);
}
// @lc code=end
