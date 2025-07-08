mod helper;

fn main() {
    let mut c = vec!['h','e','l','l','o'];
    helper::Solution::reverse_string(&mut c);
    println!("Result: {:?}", c);
}