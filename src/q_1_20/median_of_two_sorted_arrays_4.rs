struct  Solution{}
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.append(&mut nums2);
        nums1.sort();
        let len = nums1.len();
        if len%2 != 0{
            return  nums1[len/2] as f64;
        } else {
            let res = (nums1[(len/2)-1] as f64 + nums1[len/2] as f64)/2.0;
            return res;
        }
    }
}

#[test]
fn test() {
    let res = Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]);
    dbg!(res);
}