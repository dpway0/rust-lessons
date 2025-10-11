///cargo install cross
/// rustup update
/// rustup target add x86_64-unknown-linux-musl
/// rustup toolchain install stable-x86_64-unknown-linux-gnu --profile minimal --force-non-host
///cross build --release --target x86_64-unknown-linux-musl
///
/// OR
///
/// brew install zig
/// cargo install cargo-zigbuild
/// rustup target add x86_64-unknown-linux-musl
/// cargo zigbuild --release --target x86_64-unknown-linux-musl
fn main() {
    println!("Hi, dp — lightweight binary demo!");
}
