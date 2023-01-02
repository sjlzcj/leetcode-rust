pub fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = std::collections::HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        let ii = i as i32;
        if let Some(p) = seen.get(n) {
            if *p < ii {
                return vec![*p, ii];
            } else {
                return vec![ii, *p];
            }
        }
        seen.insert(target - *n, ii);
    }
    vec![]
}
