struct Solution{}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let len = nums.len();
        let mut last_elem_index = len - 1;
        let mut i = 0;
        let mut total = 0;
        if len == 0{
            return  0;
        }
        while i<len{
            while (nums[last_elem_index] == val && last_elem_index != 0){
                last_elem_index -= 1;
            }
            let v = nums[i];
            if v == val && last_elem_index != 0 && i<last_elem_index{
                let temp = v;
                nums[i] = nums[last_elem_index];
                nums[last_elem_index] = temp;
                last_elem_index = last_elem_index - 1;
            }
            i += 1;
        }
        i = 0;
        while i<len && nums[i] != val {
            total += 1;
            i += 1;
        }
        total
    }
}

#[test]
fn test() {
    let  mut nums = vec![2,4,4,4,0];
    let res = Solution::remove_element(&mut nums, 4);
    dbg!(res);
}