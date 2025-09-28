use std::env;

fn main() {
    let parts = ["api", "v1", "", "users"];
    println!("{}", join_non_empty(&parts, "/"));

    // kv.len() returns the number of bytes, not the number of Unicode characters
    let kv = "token=abc123";
    println!("{:?}", parse_kv_line(kv, '='));

    println!("{}", get_var_or("APP_ENV", "dev"));
}

fn join_non_empty(parts: &[&str], sep: &str) -> String {
    let mut out = String::new();
    let mut first = true;
    parts
        .iter()
        .filter(|p| !p.is_empty())
        .cloned()
        .for_each(|p| {
            if !first {
                out.push_str(sep); // add separator before every element after the first
            }
            out.push_str(p); // append the actual part
            first = false; // mark that we have appended the first element
        });
    out
}

fn parse_kv_line(s: &str, eq: char) -> Option<(&str, &str)> {
    let (k, v) = s.split_once(eq)?;
    let (k, v) = (k.trim(), v.trim());
    if k.is_empty() || v.is_empty() {
        None
    } else {
        Some((k, v))
    }
}

// use unwrap_or_else() to allocate only on error
fn get_var_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}
