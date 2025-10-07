use std::collections::HashMap;

// A simple trait to print a value to stdout.
// Using a trait allows a unified API across different types.
trait Printable {
    // Print the value; does not consume `self`.
    fn print(&self);
}

// Implementation for owned `String`.
impl Printable for String {
    fn print(&self) {
        println!("{}", self);
    }
}

// Implementation for integer type `i32`.
impl Printable for i32 {
    fn print(&self) {
        println!("{}", self);
    }
}

// Generic function that returns a reference to the maximum element in a slice.
// - Returns `None` if the slice is empty.
// - Requires `T: Ord` so that *every* pair of values can be compared.
fn max<T: Ord>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }

    // Start with the first element as the current maximum
    let mut max = &list[0];

    // Iterate over all elements and update `max` when we find a bigger one
    for item in list.iter() {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

// A minimal key–value storage abstraction with set/get.
trait Storage<K, V> {
    // Insert or update a value for the given key.
    fn set(&mut self, key: K, value: V);

    // Get a shared reference to the value by key.
    // Returns `None` when the key is absent.
    fn get_by_key(&self, key: &K) -> Option<&V>;
}

// Implement `Storage` for the standard `HashMap`.
// We require `K: Eq + Hash` because `HashMap` needs hashing and equality
// to place and find keys in buckets.
impl<K, V> Storage<K, V> for std::collections::HashMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn set(&mut self, key: K, value: V) {
        // `insert` returns the old value if any, which we ignore here
        self.insert(key, value);
    }

    fn get_by_key(&self, key: &K) -> Option<&V> {
        // Delegate to HashMap's `get`
        self.get(key)
    }
}

fn main() {
    // --- Printable demo ---
    let s = "Hi, dp".to_string();
    let n: i32 = 8;
    s.print();
    n.print();

    // --- max<T: Ord> demo ---
    let nums = vec![34, 50, 25, 100, 65];
    let words = vec!["alpha", "dp", "zeta", "beta"];

    println!("Max number: {:?}", max(&nums));
    println!("Max word: {:?}", max(&words));

    // --- Storage<HashMap> demo ---
    let mut store: HashMap<String, i32> = HashMap::new();
    store.set("dp".to_string(), 8);

    let v = store.get_by_key(&"dp".to_string());

    println!("Value for 'dp': {:?}", v);
}
