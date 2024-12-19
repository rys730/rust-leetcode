// pub fn is_palindrome(x: i32) -> bool {
//     if x<0{
//         return false;
//     }
//     let digits: Vec<u32> = x.to_string().chars()
//         .map(|d|d
//         .to_digit(10)
//         .unwrap())
//         .collect();
//     let reversed: Vec<u32> = digits
//         .into_iter()
//         .rev()
//         .collect();
//     let i_reversed = reversed.iter().fold(0, |acc, elem| acc * 10 + elem);

//     i_reversed as i32 == x
// }

pub fn is_palindrome(x: i32) -> bool {
    if x<0{
        return false;
    }
    let digits: u32 = x.to_string().chars()
        .map(|d|d
        .to_digit(10)
        .unwrap())
        .rev()
        .fold(0, |acc, elem| acc * 10 + elem);

    digits as i32 == x
}

#[test]
fn test() {
    let res = is_palindrome(121);
    dbg!(res);
}