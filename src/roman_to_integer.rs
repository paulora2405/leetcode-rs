/*Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000

For example, 2 is written as II in Roman numeral, just two ones added together.
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

Given a roman numeral, convert it to an integer.

Example 1:
Input: s = "III"
Output: 3
Explanation: III = 3.

Example 2:
Input: s = "LVIII"
Output: 58
Explanation: L = 50, V= 5, III = 3.

Example 3:
Input: s = "MCMXCIV"
Output: 1994
Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.

Constraints:
1 <= s.length <= 15
s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
It is guaranteed that s is a valid roman numeral in the range [1, 3999]. */
struct Solution;

#[derive(Debug)]
enum RomanNum {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use RomanNum::*;
        let mut sum = 0;
        let mut last = 0;

        for c in s.chars() {
            if c == 'I' {
                sum += I as i32;
                last = I as i32;
            } else if c == 'V' {
                if last == I as i32 {
                    sum += V as i32 - last * 2;
                } else {
                    sum += V as i32
                }
                last = V as i32;
            } else if c == 'X' {
                if last == I as i32 {
                    sum += X as i32 - last * 2;
                } else {
                    sum += X as i32
                }
                last = X as i32;
            } else if c == 'L' {
                if last == X as i32 {
                    sum += L as i32 - last * 2;
                } else {
                    sum += L as i32
                }
                last = L as i32;
            } else if c == 'C' {
                if last == X as i32 {
                    sum += C as i32 - last * 2;
                } else {
                    sum += C as i32
                }
                last = C as i32;
            } else if c == 'D' {
                if last == C as i32 {
                    sum += D as i32 - last * 2;
                } else {
                    sum += D as i32
                }
                last = D as i32;
            } else if c == 'M' {
                if last == C as i32 {
                    sum += M as i32 - last * 2;
                } else {
                    sum += M as i32
                }
                last = M as i32;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = String::from("III");
        let result = Solution::roman_to_int(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let input = String::from("LVIII");
        let result = Solution::roman_to_int(input);
        assert_eq!(result, 58);
    }

    #[test]
    fn example_3() {
        let input = String::from("MCMXCIV");
        let result = Solution::roman_to_int(input);
        assert_eq!(result, 1994);
    }
}
