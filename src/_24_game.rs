/* You are given an integer array cards of length 4.
You have four cards, each containing a number in the range [1, 9].
You should arrange the numbers on these cards in a mathematical expression using
the operators ['+', '-', '*', '/'] and the parentheses '(' and ')' to get the value 24.

You are restricted with the following rules:

The division operator '/' represents real division, not integer division.
    For example, 4 / (1 - 2 / 3) = 4 / (1 / 3) = 12.
Every operation done is between two numbers. In particular, we cannot use '-' as a unary operator.
    For example, if cards = [1, 1, 1, 1], the expression "-1 - 1 - 1 - 1" is not allowed.
You cannot concatenate numbers together
    For example, if cards = [1, 2, 1, 2], the expression "12 + 12" is not valid.
Return true if you can get such expression that evaluates to 24, and false otherwise.

Example 1:
Input: cards = [4,1,8,7]
Output: true
Explanation: (8-4) * (7-1) = 24

Example 2:
Input: cards = [1,2,1,2]
Output: false

Constraints:
cards.length == 4
1 <= cards[i] <= 9 */

struct Solution;

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let mut cards = cards.into_iter().map(|n| n as f32).collect::<Vec<f32>>();
        has_reached_result(&cards)
    }
}

fn has_reached_result(cards: &[f32]) -> bool {
    let len = cards.len();
    if len == 1 {
        return (cards[0] - 24.0).abs() <= 0.1;
    }
    for i in 0..len {
        for j in (i + 1)..len {
            let mut new_cards = Vec::with_capacity(4);
            for k in (0..len) {
                if k != i && k != j {
                    new_cards.push(cards[k]);
                }
            }
            for res in generate_possible_results(cards[i], cards[j]) {
                new_cards.push(res);
                if has_reached_result(&new_cards) {
                    return true;
                }
                new_cards.pop();
            }
        }
    }
    return false;
}

fn generate_possible_results(a: f32, b: f32) -> Vec<f32> {
    let mut possible = vec![a + b, a - b, b - a, a * b];
    if a != 0.0 {
        possible.push(b / a);
    }
    if b != 0.0 {
        possible.push(a / b);
    }
    possible
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn example_1() {
        let input = vec![4, 1, 8, 7];
        assert_eq!(Solution::judge_point24(input), true);
    }

    #[test]
    fn example_2() {
        let input = vec![1, 2, 1, 2];
        assert_eq!(Solution::judge_point24(input), false);
    }
}
