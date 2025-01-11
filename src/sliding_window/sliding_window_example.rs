use std::cmp::{max_by};

impl SlidingWindow {
    // find longest len subarray with sum less than or eq to constraint
    pub fn find_longest_subarray(nums: Vec<i32>, constraint: i32) -> usize{
        let (mut left, mut curr) = (0, 0);
        let mut answer = 0;
        for right in 0..nums.len()  {
            curr += nums[right];
            while curr > constraint {
                curr -= nums[left];
                left += 1;
            }
            answer = max_by(answer, (right - left + 1), |x, y|x.cmp(&y));
        }
        answer
    }
}

struct SlidingWindow{}

#[test]
fn test() {
    let test_case = vec![3, 2, 1, 3, 1, 1];
    let constraint = 5;
    let res = SlidingWindow::find_longest_subarray(test_case, constraint);
    dbg!(res);
}