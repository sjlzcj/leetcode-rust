use std::collections::HashMap;

pub fn solution(s: String) -> i32 {
    let (mut max, mut map, mut seq_start_index) = (0, [-1; 128], -1);
    let s = s.into_bytes();
    for i in 0..s.len() {
        if map[s[i] as usize] >= seq_start_index {
            max = max.max(i as i32 - seq_start_index);
            seq_start_index = map[s[i] as usize] + 1;
        } else {
            max = max.max(i as i32 - seq_start_index + 1);
        }
        map[s[i] as usize] = i as i32;
    }
    max
}
