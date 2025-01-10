impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let parts = s.split_whitespace();
        let len = parts.clone().count() -1;
        dbg!(&len);
        for (i, c) in parts.enumerate(){
             if i == len{
                return  c.len() as i32;
             }
        }
       1
    }
}

// best solution (not mine)
// impl Solution {
//     pub fn length_of_last_word(s: String) -> i32 {
//         s.split_whitespace().last().unwrap_or_default().len() as i32
//     }
// }

struct Solution{}

#[test]
fn test() {
    let res = Solution::length_of_last_word(String::from("Hello World "));
}