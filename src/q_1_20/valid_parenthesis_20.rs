use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let map = HashMap::from([
            ('(', ')'),
            ('{', '}'),
            ('[', ']'),
        ]);
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars(){
            if map.contains_key(&c){
                stack.push(c);
            } else {
                let _c = stack.pop();
                match _c {
                    Some(_c) => {
                        let &_match = map.get(&_c).unwrap();
                        if _match != c{
                            return false;
                        }
                    }
                    None => return  false
                }
            }
        }
        if stack.len()!=0{
            return  false;
        }
        true
    }
}

struct Solution{}

#[test]
fn test() {
    let res = Solution::is_valid(String::from("]"));
    dbg!(res);
}