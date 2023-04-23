struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut unique_nums = std::collections::BTreeSet::new();
        for n in nums.iter() {
            unique_nums.insert(*n);
        }
        let distinct = unique_nums.len();
        unique_nums
            .into_iter()
            .enumerate()
            .for_each(|(i, n)| nums.insert(i, n));

        distinct as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn example_1() {
        let mut input = vec![1, 1, 2];
        let result = Solution::remove_duplicates(&mut input);
        let expected_vec = vec![1, 2];
        assert_eq!(result, expected_vec.len() as i32);
        assert_eq!(input[..expected_vec.len()], expected_vec);
    }

    #[test]
    fn example_2() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

        let result = Solution::remove_duplicates(&mut input);
        let expected_vec = vec![0, 1, 2, 3, 4];
        assert_eq!(result, expected_vec.len() as i32);
        assert_eq!(input[..expected_vec.len()], expected_vec);
    }
}
