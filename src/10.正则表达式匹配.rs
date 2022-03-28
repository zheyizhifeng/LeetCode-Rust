/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 */
// @test("aa","a")=false
// @test("aa","a*")=true
// @test("ab",".*")=true
// @test("aab", "c*a*b")=true
// @lc code=start
// struct Solution {}
impl Solution {
    pub fn is_char_match(s: u8, p: u8) -> bool {
        s == p || p == b'.'
    }
    pub fn is_match(s: String, p: String) -> bool {
        let (s, p, len_s, len_p) = (s.as_bytes(), p.as_bytes(), s.len(), p.len());
        let mut dp = vec![vec![false; len_p + 1]; len_s + 1];

        dp[0][0] = true;

        // badcase
        for j in 1..=len_p {
            if p[j - 1] == b'*' {
                dp[0][j] = dp[0][j - 2];
            }
        }
        for i in 1..=len_s {
            for j in 1..=len_p {
                if Solution::is_char_match(s[i - 1], p[j - 1]) {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if p[j - 1] == b'*' && j >= 2 {
                    if Solution::is_char_match(s[i - 1], p[j - 2]) {
                        dp[i][j] = dp[i][j - 2] || dp[i - 1][j - 2] || dp[i - 1][j];
                    } else {
                        dp[i][j] = dp[i][j - 2];
                    }
                }
            }
        }
        dp[len_s][len_p]
    }
}
// fn main() {
//     println!("ans is {}", Solution::is_match("aa".to_string(), "a".to_string()));
//     println!("ans is {}", Solution::is_match("aa".to_string(), "a*".to_string()));
//     println!("ans is {}", Solution::is_match("ab".to_string(), ".*".to_string()));
//     println!("ans is {}", Solution::is_match("aab".to_string(), "c*a*b".to_string()));
// }
// @lc code=end
