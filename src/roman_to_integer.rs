#![allow(dead_code, unused)]
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
