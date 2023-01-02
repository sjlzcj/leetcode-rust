mod two_sum;
fn main() {
    let nums = vec![1, 2, 5, 4, 3];
    let results = two_sum::solution(nums, 8);
    for ele in results {
        println!("{}", ele);
    }
}