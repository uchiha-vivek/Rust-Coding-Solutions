pub struct Solution;



// method 1 : Brute force method
// Time complexity

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum: i32 = std::i32::MIN;

        for i in 0..nums.len() {
            for j in i..nums.len() {
                let mut sum: i32 = 0;
                for k in i..=j {
                    sum += nums[k];
                }
                // 🔥 THIS WAS MISSING
                max_sum = max_sum.max(sum);
            }
        }

        max_sum
    }

    // if array is  [a,b,c,d] where a,b,c,d are integers
    // then  sum looks like a, a+b, a+b+c, a+b+c+d then b, b+c, b+c+d then c, c+d then d
    // time complexity : O(n^2)
    // space complexity : O(1)

    pub fn max_sub_array_method2(nums: Vec<i32>) -> i32 {
        let mut max_sum  = i32::MIN;
        for i in 0..nums.len(){
            let mut current_sum= 0;
            for j in i..nums.len(){
                current_sum+= nums[j];
                max_sum = max_sum.max(current_sum);
            }
        }
        max_sum
    }

}
