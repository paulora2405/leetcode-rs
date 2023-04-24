/* Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
For example, 2 is written as II in Roman numeral, just two one's added together.
12 is written as XII, which is simply X + II.
The number 27 is written as XXVII, which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right.
However, the numeral for four is not IIII. Instead, the number four is written as IV.
Because the one is before the five we subtract it making four.
The same principle applies to the number nine, which is written as IX.
There are six instances where subtraction is used:
    I can be placed before V (5) and X (10) to make 4 and 9.
    X can be placed before L (50) and C (100) to make 40 and 90.
    C can be placed before D (500) and M (1000) to make 400 and 900.

Given an integer, convert it to a roman numeral.

Example 1:
Input: num = 3
Output: "III"
Explanation: 3 is represented as 3 ones.

Example 2:
Input: num = 58
Output: "LVIII"
Explanation: L = 50, V = 5, III = 3.

Example 3:
Input: num = 1994
Output: "MCMXCIV"
Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.

Constraints:
1 <= num <= 3999 */
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
