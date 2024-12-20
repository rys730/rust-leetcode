struct Solution{}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let strs_len = strs.len();
        let mut prefix = Vec::new();
        let max_length = strs.clone().iter().map(|s| s.len()).max().unwrap_or(0);
        for i in 0..max_length{
            let mut same_char = Vec::new();
            for s in &strs{
                match s.chars().nth(i) {
                    Some(c) => same_char.push(c),
                    None => break
                }
            }
            if same_char.iter().all(|&x| x == same_char[0]) && same_char.len() == strs_len{
                prefix.push(same_char[0]);
            } else {
                break;
            }
        }
        prefix.iter().collect() 
    }
}

#[test]
fn test() {
    let i: Vec<String> = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
    let res = Solution::longest_common_prefix(i);
    dbg!(res);
}