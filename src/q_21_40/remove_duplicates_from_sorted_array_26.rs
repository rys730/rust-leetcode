impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut index = 0;
        for n in 1..nums.len(){
            if nums[n] != nums[index]{
                index += 1;
                nums[index] = nums[n];
            }
        }
        return (index+1) as i32;
    }
}

struct Solution{}

#[test]
fn test() {
    
}