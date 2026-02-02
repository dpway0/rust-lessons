use std::collections::HashMap;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    match two_sum(nums, target) {
        Some(indices) => println!("Found indices: {:?}", indices),
        None => println!("No solution found"),
    }

    let s = "A man, a plan, a canal: Panama";
    
    if is_alphanumeric_palindrome(s) {
        println!("'{}' is a palindrome", s);
    } else {
        println!("'{}' is not a palindrome", s);
    }

    let arr = vec![17, 18, 5, 4, 6, 1];
    let result = replace_elements(arr);
    println!("Result replace elements: {:?}", result);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Option<Vec<i32>> {
    let mut pre_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&pre_index) = pre_map.get(&complement) {
            return Some(vec![pre_index, i as i32]);
        }

        pre_map.insert(num, i as i32);
    }
    
    None
}

fn is_alphanumeric_palindrome(s: &str) -> bool {

    let bytes = s.as_bytes();

    let (mut left, mut right) = (0, bytes.len() as i32 - 1);

    while left < right {
        if !bytes[left as usize].is_ascii_alphanumeric() {
            left += 1;
            continue;
        } 

        if !bytes[right as usize].is_ascii_alphanumeric() {
            right -= 1;
            continue;
        }

        if bytes[left as usize].to_ascii_lowercase() != bytes[right as usize].to_ascii_lowercase() {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}

fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
    let mut max_val = -1;

    for i in (0..arr.len()).rev() {
        let current_val = arr[i];
        arr[i] = max_val;

        max_val = max_val.max(current_val);
    }

    arr
}