#![allow(dead_code, unused)]

struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut ret = String::new();
        let mut num = num;
        let mut one = num % 10;
        num /= 10;
        let mut ten = num % 10;
        num /= 10;
        let mut hundred = num % 10;
        num /= 10;
        let mut thousand = num % 10;

        for _ in 0..thousand {
            ret.push('M');
        }

        if hundred == 4 {
            ret.push_str("CD");
        } else if hundred == 9 {
            ret.push_str("CM");
        } else if hundred >= 5 {
            ret.push('D');
            for _ in 5..hundred {
                ret.push('C');
            }
        } else if hundred <= 3 {
            for _ in 0..hundred {
                ret.push('C');
            }
        }

        if ten == 4 {
            ret.push_str("XL");
        } else if ten == 9 {
            ret.push_str("XC");
        } else if ten >= 5 {
            ret.push('L');
            for _ in 5..ten {
                ret.push('X');
            }
        } else if ten <= 3 {
            for _ in 0..ten {
                ret.push('X');
            }
        }

        if one == 4 {
            ret.push_str("IV");
        } else if one == 9 {
            ret.push_str("IX");
        } else if one >= 5 {
            ret.push('V');
            for _ in 5..one {
                ret.push('I');
            }
        } else if one <= 3 {
            for _ in 0..one {
                ret.push('I');
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = 3;
        let expected = String::from("III");
        let result = Solution::int_to_roman(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let input = 58;
        let expected = String::from("LVIII");
        let result = Solution::int_to_roman(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn example_3() {
        let input = 1994;
        let expected = String::from("MCMXCIV");
        let result = Solution::int_to_roman(input);
        assert_eq!(result, expected);
    }
}
