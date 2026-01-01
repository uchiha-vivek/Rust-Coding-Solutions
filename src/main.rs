// use rust_code_solutions::solutions::strings::valid_anagram::Solution;
// use rust_code_solutions::solutions::arrays::max_sub_array::Solution;
use rust_code_solutions::solutions::stack::custom_stack::Stack;

fn main() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);
    print!("Top: {:?}",stack.peek());

}
