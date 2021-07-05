impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut defanged_address = String::new();

        for c in address.chars() {
            let defanged_item: String = match c {
                '.' => "[.]".to_string(),
                _ => c.to_string(),
            };

            defanged_address.push_str(&defanged_item);
        }

        defanged_address
    }
}
