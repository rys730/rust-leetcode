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

    // "1101100111"
    pub fn longest_substring_all_one(s: String) -> usize{
        let mut left = 0;
        let mut ans = 0;
        let mut total_zero = 0;
        let vec: Vec<char> = s.chars().collect();
        dbg!(&vec);
        for right in 0..vec.len(){
            let curr = vec[right];
            if curr == '0'{
                total_zero += 1;
            }
            while total_zero > 1 {
                if vec[left] == '0'{
                    total_zero -= 1;
                }
                left += 1;
            }
            dbg!(&right);
            dbg!(&left);
            ans = max_by(ans, (right + 1 - left) as i32, |x, y| x.cmp(&y));
        }
        ans as usize
    }

    // [10, 5, 2, 6]
    pub fn subarray_product(nums: Vec<i32>, constraint: i32) -> usize{
        let mut left = 0;
        let mut ans = 0;
        let mut curr = 1;
        for right in 0..nums.len(){
            curr *= nums[right];
            while curr >= constraint {
                curr /= nums[left];
                left += 1;
            }
            ans += right  - left + 1;
        }
        ans
    }


    pub fn fixed_window_size(nums: Vec<i32>, size: usize) -> usize{
        let mut curr = 0;
        let mut ans = 0;
        // get first batch result
        for i in 0..size  {
            ans += nums[i];
        }
        curr = ans;
        for j in size..nums.len(){
            curr += nums[j] - nums[j-size];
            ans = max_by(ans, curr, |x, y| x.cmp(&y));
        }
        ans as usize
    }
}

struct SlidingWindow{}

#[test]
fn test() {
    let test_case = vec![3, 1, 2, 7, 4, 2, 1, 1, 5];
    let constraint = 8;
    let res = SlidingWindow::find_longest_subarray(test_case, constraint);
    dbg!(res);
}



// Example 2: You are given a binary string s (a string containing only "0" and "1"). You may choose up to one "0" and flip it to a "1". What is the length of the longest substring achievable that contains only "1"?
// For example, given s = "1101100111", the answer is 5. If you perform the flip at index 2, the string becomes 1111100111.

#[test]
fn test2() {
    let test_case = String::from("1101100111");
    let res = SlidingWindow::longest_substring_all_one(test_case);
    dbg!(res);
}




// Example 3: 713. Subarray Product Less Than K.

// Given an array of positive integers nums and an integer k, return the number of subarrays where the product of all the elements in the subarray is strictly less than k.

// For example, given the input nums = [10, 5, 2, 6], k = 100, the answer is 8. The subarrays with products less than k are:

// [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]

#[test]
fn test3() {
    let test_case = vec![10, 5, 2, 6];
    let constraint = 100;
    let res = SlidingWindow::subarray_product(test_case, constraint);
    dbg!(res);
}

// Example 4: Given an integer array nums and an integer k, find the sum of the subarray with the largest sum whose length is k.
#[test]
fn test4() {
    let test_case = vec![1,20,3,4];
    let constraint = 2;
    let res = SlidingWindow::fixed_window_size(test_case, constraint);
    dbg!(res);
}