use std::collections::{HashMap, HashSet};

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result = top_k_frequent(nums, k);
    println!("Top {} frequent elements: {:?}", k, result); // Expected: [1, 2]

    let nums = vec![1, 2, 3, 4];
    let result = product_except_self(nums);
    println!("Product except self: {:?}", result); // Expected: [24, 12, 8, 6]

    let nums = vec![100, 4, 200, 1, 3, 2];
    let result = longest_consecutive(nums);
    println!("Longest consecutive length: {}", result); // Expected: 4
}

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(k as usize);
    let n = nums.len();
    let mut count_map: HashMap<i32, i32> = HashMap::with_capacity(n);
    for num in nums {
        *count_map.entry(num).or_insert(0) += 1;
    }

    let mut buckets: Vec<Vec<i32>> = vec![vec![]; n + 1];
    for (num, freq) in count_map {
        buckets[freq as usize].push(num);
    }

    for freq_list in buckets.into_iter().rev() {
        for num in freq_list {
            result.push(num);
            if result.len() == k as usize {
                return result;
            }
        }
    }

    result
}

fn product_except_self(nums: Vec<i32>) -> Vec<i32>{
    let n = nums.len();
    let mut res = vec![1;n];

    let mut prefix = 1;
    for i in 0..n {
        res[i] = prefix;
        prefix *= nums[i];
    }

    let mut suffix = 1;
    for i in (0..n).rev() {
        res[i] *= suffix;
        suffix *= nums[i]; 
    }
    res
}

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut longest = 0;
    let num_set: HashSet<i32> = nums.into_iter().collect();

    for &num in &num_set {
        if !num_set.contains(&(num - 1)) {
            let mut current_num = num;
            let mut current_streak = 1;
            while num_set.contains(&(current_num + 1)) {
                current_num += 1;
                current_streak += 1;
            }

            longest = longest.max(current_streak);
        }
    }
    longest
}