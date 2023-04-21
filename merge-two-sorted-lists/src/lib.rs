#![allow(dead_code, unused)]

struct Solution;

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

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut heap = std::collections::BinaryHeap::<i32>::new();

        let mut l1 = list1;
        while let Some(item) = l1 {
            heap.push(item.val);
            l1 = item.next;
        }

        let mut l2 = list2;
        while let Some(item) = l2 {
            heap.push(item.val);
            l2 = item.next;
        }

        let mut res = None;
        while let Some(item) = heap.pop() {
            res = Some(Box::new(ListNode {
                val: item,
                next: res,
            }));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn example_1() {
        let l1 = ListNode::from_vec(&mut [1, 2, 4]);
        let l2 = ListNode::from_vec(&mut [1, 3, 4]);

        let result = Solution::merge_two_lists(l1, l2);

        let expected: Option<Box<ListNode>> = ListNode::from_vec(&mut [1, 1, 2, 3, 4, 4]);

        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let l1 = ListNode::from_vec(&mut []);
        let l2 = ListNode::from_vec(&mut []);

        let result = Solution::merge_two_lists(l1, l2);

        let expected: Option<Box<ListNode>> = ListNode::from_vec(&mut []);

        assert_eq!(result, expected);
    }

    #[test]
    fn example_3() {
        let l1 = ListNode::from_vec(&mut []);
        let l2 = ListNode::from_vec(&mut [0]);

        let result = Solution::merge_two_lists(l1, l2);

        let expected: Option<Box<ListNode>> = ListNode::from_vec(&mut [0]);

        assert_eq!(result, expected);
    }
}
