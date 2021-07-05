use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut jewel_types = HashSet::new();
        let mut jewels_count: i32 = 0;

        for jewel_type in jewels.chars() {
            jewel_types.insert(jewel_type);
        }

        for stone_type in stones.chars() {
            if jewel_types.contains(&stone_type) {
                jewels_count += 1;
            }
        }

        jewels_count
    }
}
