#![allow(dead_code, unused)]

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn back_track(s: String, open: i32, close: i32) -> Vec<String> {
            dbg!(&s);
            let mut res = vec![];
            if open == 0 && close == 0 {
                return vec![s];
            }
            if open > 0 {
                res.append(&mut back_track(s.clone() + "(", open - 1, close + 1));
            }
            if close > 0 {
                res.append(&mut back_track(s + ")", open, close - 1));
            }
            res
        }
        back_track("".to_string(), n, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn example_1() {
        let input = 3;
        let expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        let result = Solution::generate_parenthesis(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let input = 1;
        let expected = vec!["()"];
        let result = Solution::generate_parenthesis(input);
        assert_eq!(result, expected);
    }
}
