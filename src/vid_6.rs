#![allow(dead_code)]
use std::collections::HashMap;

// ** START EDITS HERE **

fn max_profit(prices: &[i32]) -> i32 {
    // A Leetcode favorite (slightly altered): https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/
    // Feel free to solve either here instead of leetcode for editor tooltips, cargo check help, and a little bit of
    // practice with references as parameters
    // Extra practice: Try utilizing match, .cmp() and/or .max() methods (found in std::cmp module) in your implementation
    let mut buy = i32::MAX;
    let mut max_profit = 0;
    for price in prices {
        let price = *price; // Dereference price, shadow original price variable
        match price.cmp(&buy) {
            std::cmp::Ordering::Less => buy = price,
            std::cmp::Ordering::Greater => max_profit = max_profit.max(price - buy),
            _ => continue,
        }
    }

    max_profit
}

fn blue_42(hm: &mut HashMap<String, i32>) {
    // For every key within this hashmap, if the key starts with "blue" (lowercase),
    // add 42 to the key's value
    // Utilize the docs if needed!
    for (key, val) in hm {
        if key.starts_with("blue") {
            *val += 42;
        }
    }
}

// ** END EDITS HERE **

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    #[test]
    fn max_profit() {
        let testcases = [
            ([7, 1, 5, 3, 6, 4], 5),
            ([7, 6, 4, 3, 1, 1], 0),
            ([10, 3, 5, 6, 7, 1], 4),
            ([2, 4, 12, 1, 15, 16], 15),
            ([1, 4, 4, 4, 5, 6], 5),
        ];
        for (tc, verify) in &testcases {
            assert_eq!(super::max_profit(tc), *verify);
        }
    }

    #[test]
    fn blue_42() {
        let mut hm = HashMap::new();
        hm.insert("blueish".to_string(), 0);
        hm.insert("green".to_string(), 42);
        hm.insert("boo".to_string(), 1);
        hm.insert("blue".to_string(), 8);

        super::blue_42(&mut hm);

        assert_eq!(hm["blueish"], 42);
        assert_eq!(hm["green"], 42);
        assert_eq!(hm["boo"], 1);
        assert_eq!(hm["blue"], 50);
    }
}
