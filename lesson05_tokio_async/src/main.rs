use std::time::Duration;

use anyhow::{Context, Result};
use reqwest::Client;
use tokio::time::{sleep, timeout};

async fn say_after(msg: &str, delay_ms: u64) {
    sleep(Duration::from_millis(delay_ms)).await;
    println!("{msg}");
}

async fn worker(name: &str, delay_ms: u64) {
    for i in 1..=5 {
        println!("[{name}] line {i}");
        sleep(Duration::from_millis(delay_ms)).await;
    }
    println!("[{name}] done")
}

// Fetch the entire body and return its length in bytes
async fn fetch_len(client: &Client, url: &str) -> Result<usize> {
    // GET request
    let resp = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("request failed: GET {url}"))?;

    // Read the body fully into memory
    let status = resp.status();
    let body = resp
        .bytes()
        .await
        .with_context(|| format!("read body failed: {url}"))?;

    // Treat non-2xx as an error
    if !status.is_success() {
        anyhow::bail!("{url} -> HTTP {status}");
    }

    Ok(body.len())
}

// Wrap a single fetch with a timeout and return (url, result).
async fn fetch_task(client: &Client, url: &str) -> (String, Result<usize>) {
    let fut = fetch_len(&client, url);
    let res = timeout(Duration::from_secs(10), fut)
        .await
        .map_err(|_| anyhow::anyhow!("timeout"))
        .flatten();

    (url.to_string(), res)
}

#[tokio::main]
async fn main() -> Result<()> {
    say_after("Hi after 1s", 1000).await;
    say_after("Hi after 2s", 2000).await;

    let t1 = worker("A (200ms)", 200);
    let t2 = worker("B (300ms)", 300);
    let t3 = worker("C (600ms)", 600);

    // Run all three concurrently and wait for all to finish
    tokio::join!(t1, t2, t3);

    println!("All tasks finished");

    // Build a reqwest client with a UA and a small pool idle timeout
    let client = Client::builder()
        .pool_idle_timeout(Duration::from_secs(15))
        .build()?;

    // Three sample URLs — replace with your own
    let u1 = "https://example.com";
    let u2 = "https://www.rust-lang.org/";
    let u3 = "https://httpbin.org/bytes/2048";

    // Run all three tasks concurrently and wait for all to finish
    let (r1, r2, r3) = tokio::join!(
        fetch_task(&client, u1),
        fetch_task(&client, u2),
        fetch_task(&client, u3),
    );

    // Print each result (length or error)
    for (url, res) in [r1, r2, r3] {
        match res {
            Ok(len) => println!("{url} -> {len} bytes"),
            Err(e) => eprintln!("{url} -> ERROR: {e:#}"),
        }
    }

    Ok(())
}
