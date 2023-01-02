pub fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = std::collections::HashMap::new();

    for (i, n) in nums.iter().enumerate() {
        if let Some(p) = seen.get(n) {
            return vec![*p, i as i32];
        }
        seen.insert(target - *n, i as i32);
    }
    vec![]
}