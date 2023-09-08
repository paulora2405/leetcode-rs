/* Given the head of a singly linked list, reverse the list, and return the reversed list.

Example 1:
Input: head = [1,2,3,4,5]
Output: [5,4,3,2,1]

Example 2:
Input: head = [1,2]
Output: [2,1]

Example 3:
Input: head = []
Output: []

Constraints:
The number of nodes in the list is the range [0, 5000].
-5000 <= Node.val <= 5000

Follow up: A linked list can be reversed either iteratively or recursively. Could you implement both? */

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_vec(vals: &mut [i32]) -> Option<Box<Self>> {
        if vals.is_empty() {
            return None;
        }
        vals.reverse();
        let mut res = None;
        for val in vals {
            res = Some(Box::new(ListNode {
                val: *val,
                next: res,
            }))
        }
        res
    }
}

struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut curr) = (None, head);
        while let Some(mut node) = curr {
            curr = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn example_1() {
        let input = ListNode::from_vec(&mut [1, 2, 3, 4, 5]);
        let result = Solution::reverse_list(input);
        let expected = ListNode::from_vec(&mut [5, 4, 3, 2, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let input = ListNode::from_vec(&mut [1, 2]);
        let result = Solution::reverse_list(input);
        let expected = ListNode::from_vec(&mut [2, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn example_3() {
        let input = ListNode::from_vec(&mut []);
        let result = Solution::reverse_list(input);
        let expected = ListNode::from_vec(&mut []);
        assert_eq!(result, expected);
    }
}
