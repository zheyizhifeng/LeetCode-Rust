/*
 * @lc app=leetcode.cn id=28 lang=rust
 *
 * [28] 实现 strStr()
 */

// @lc code=start
impl Solution {
  pub fn getPMT(pattern: &[u8]) -> Vec<usize> {
    let mut i = 1;
    let mut j = 0;
    let pattern_len = pattern.len();
    let mut PMT = vec![0; pattern_len];
    while i < pattern.len() {
      if pattern[i] == pattern[j] {
        j += 1;
        PMT[i] = j;
        i += 1;
      } else {
        if j > 0 {
          j = PMT[j - 1];
        } else {
          i += 1;
        }
      }
    }
    PMT
  }
  pub fn str_str(haystack: String, needle: String) -> i32 {
    let (len_haystack, len_needle) = (haystack.len(), needle.len());
    if len_needle == 0 || needle == haystack {
      return 0;
    }
    let (haystack, needle) = (haystack.as_bytes(), needle.as_bytes());
    let (mut i, mut j) = (0, 0);
    let PMT = Solution::getPMT(&needle);
    while i < len_haystack {
      if haystack[i] == needle[j] {
        i += 1;
        j += 1;
        if j == len_needle {
          return (i - j) as i32;
        }
      } else {
        if j > 0 {
          j = PMT[j - 1];
        } else {
          i += 1;
        }
      }
    }
    -1
  }
}
// @lc code=end
