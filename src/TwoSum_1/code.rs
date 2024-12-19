use std::collections::HashMap;

// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     for (i, &i_v) in nums.iter().enumerate(){
//         for j in 0..nums.len(){
//             if i == j {
//                 continue;
//             }
//             if i_v + nums[j] == target{
//                 return vec![i as i32, j as i32];
//             }
//         }
//     }
//     return vec![0,0];
// }

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut v_map: HashMap<i32, usize> = HashMap::new();
    for (i, &i_v) in nums.iter().enumerate(){
        v_map.insert(i_v, i);
    }
    for (i, _) in nums.iter().enumerate(){
        let val = target - nums[i];
        if let Some(&index) = v_map.get(&val){
            if index != i{
                return vec![i as i32, index as i32];
            }
        }
    }
    return vec![0,0];
}

#[test]
fn test() {
    let res = two_sum(vec![3,2,4], 6);
    dbg!(res);
}