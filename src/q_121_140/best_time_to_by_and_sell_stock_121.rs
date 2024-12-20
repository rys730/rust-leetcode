impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut buy = prices[0];
        
        for &p in prices.iter(){
            if p - buy > max_profit{
                max_profit = p-buy;
            }
            if p < buy{
                buy = p;
            }
        }

        max_profit
    }
}
struct Solution{}

#[test]
fn test() {
    let prices = vec![7,1,5,3,6,4];
    let res = Solution::max_profit(prices); 
    dbg!(res);
}