use std::collections::HashMap;

struct Solution{}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map: HashMap<char, i32> = vec![
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .into_iter()
        .collect();
        let mut last_val = 0;
        let mut sum = 0;
        for c in s.chars(){
            if let Some(&v) = map.get(&c){
                if v > last_val{
                    sum += v - last_val - last_val;
                } else {
                    sum += v;
                }
                last_val = v;
            }
        }
        sum
    }
}

#[test]
fn test() {
    let res = Solution::roman_to_int(String::from("IV"));
    dbg!(res);
}