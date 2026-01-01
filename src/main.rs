// use rust_code_solutions::solutions::strings::valid_anagram::Solution;
use rust_code_solutions::solutions::arrays::max_sub_array::Solution;

fn main() {
     
    let nums = vec![5,4,-1,7,8];
    let result = Solution::max_sub_array(nums);
    println!("Result: {}", result);
}
