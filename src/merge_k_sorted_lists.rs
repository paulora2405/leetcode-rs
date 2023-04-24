/* You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
Merge all the linked-lists into one sorted linked-list and return it.

Example 1:
Input: lists = [[1,4,5],[1,3,4],[2,6]]
Output: [1,1,2,3,4,4,5,6]
Explanation: The linked-lists are:
[
  1->4->5,
  1->3->4,
  2->6
]
merging them into one sorted list:
1->1->2->3->4->4->5->6

Example 2:
Input: lists = []
Output: []

Example 3:
Input: lists = [[]]
Output: []

Constraints:
k == lists.length
0 <= k <= 10^4
0 <= lists[i].length <= 500
-10^4 <= lists[i][j] <= 10^4
lists[i] is sorted in ascending order.
The sum of lists[i].length will not exceed 10^4. */

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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = std::collections::BinaryHeap::new();
        for mut next in lists {
            while let Some(item) = next {
                heap.push(item.val);
                next = item.next;
            }
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
        let l1 = ListNode::from_vec(&mut [1, 4, 5]);
        let l2 = ListNode::from_vec(&mut [1, 3, 4]);
        let l3 = ListNode::from_vec(&mut [2, 6]);
        let input: Vec<Option<Box<ListNode>>> = vec![l1, l2, l3];
        let expected: Option<Box<ListNode>> = ListNode::from_vec(&mut [1, 1, 2, 3, 4, 4, 5, 6]);
        let result = Solution::merge_k_lists(input);

        assert_eq!(result, expected);
    }
}
