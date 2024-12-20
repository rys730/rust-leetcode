use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut h1 = height[0];
        let mut h2 = height[len-1];
        let (mut i1, mut i2) = (0 as i32, (len-1) as i32);
        let mut area = 0;
        while  i1 != i2 {
            let _area =  cmp::min(h1, h2) * (i2-i1);
            if _area > area{
                area = _area;
            }
            if h1 < h2 {
                i1 += 1;
                h1 = height[i1 as usize];
            } else {
                i2 -= 1;
                h2 = height[i2 as usize];
            }
        } 
        area
    }
}
struct Solution{}

#[test]
fn test() {
    let i = vec![1,8,6,2,5,4,8,3,7];
    let res = Solution::max_area(i);
    dbg!(res);
}