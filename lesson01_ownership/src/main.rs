struct Counter {
    value: i32,
}

impl Counter {
    fn inc(&mut self) {
        self.value += 1;
    }
    fn get(&self) -> i32 {
        self.value
    }
}

fn main() {
    let s = String::from("Hi, dp");
    let c = first_char(&s).unwrap_or('_'); // remember: char uses single quotes
    println!("s first (default): {}", c);

    let mut counter = Counter { value: 32 };
    counter.inc();
    println!("Value after inc: {}", counter.get());
}

fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}
