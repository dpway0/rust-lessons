use std::{
    error::Error,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

fn spawn_logger(rx: Receiver<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let Ok(msg) = rx.recv() else {
                println!("[logger] channel closed; exiting");
                break;
            };
            println!("{msg}");
        }
    })
}

fn spawn_workers(
    n: usize,
    jobs_rx: Arc<Mutex<mpsc::Receiver<i32>>>,
    results_tx: mpsc::Sender<(usize, i32, i32)>,
) -> Vec<thread::JoinHandle<()>> {
    (0..n)
        .map(|wid| {
            let rx = Arc::clone(&jobs_rx);
            let tx = results_tx.clone();
            thread::spawn(move || {
                loop {
                    // Take one job while holding the lock; release the lock before computing.
                    let x = {
                        // Acquire the mutex; exit if poisoned.
                        let Ok(guard) = rx.lock() else {
                            eprintln!("[worker {wid}] mutex poisoned; exiting");
                            break;
                        };
                        // Block for the next job; exit when the jobs channel is closed.
                        let Ok(val) = guard.recv() else { break };
                        val
                    }; // guard is dropped here → other workers can receive next job

                    let y = x * x;

                    // Send result; if the sink is closed, exit gracefully.
                    let Ok(()) = tx.send((wid, x, y)) else {
                        eprintln!("[worker {wid}] result sink closed; exiting");
                        break;
                    };
                }
            })
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel::<String>();
    // Helper: spawn a worker thread that sends 5 messages
    let spawn_worker = |name: &'static str, tx_main: Sender<String>| {
        thread::spawn(move || {
            for i in 1..=5 {
                let msg = format!("{name} -> message #{i}");
                // Early-exit on send failure
                let Ok(()) = tx_main.send(msg) else {
                    println!("[{name}] send failed (receiver closed)");
                    break;
                };
                thread::sleep(Duration::from_millis(40));
            }
        })
    };

    // Start the logger thread
    let logger = spawn_logger(rx);

    let h1 = spawn_worker("T1", tx.clone());
    let h2 = spawn_worker("T2", tx.clone());

    //Main may also send logs
    for i in 1..=3 {
        let Ok(()) = tx.send(format!("[main] boot step {i}")) else {
            println!("[main] send failed");
            break;
        };
    }

    // Ensure receiver can finish when workers are done
    drop(tx);

    // Receive until channel closes (early-break on error)
    // loop {
    //     let Ok(msg) = rx.recv() else {
    //         println!("[main] channel closed; draining done");
    //         break;
    //     };
    //     println!("{msg}");
    // }

    // Join workers (early-return on panic)
    let Ok(_) = h1.join() else {
        return Err("T1 panicked".into());
    };
    let Ok(_) = h2.join() else {
        return Err("T2 panicked".into());
    };

    // Join logger (ensures all logs flushed)
    let Ok(_) = logger.join() else {
        return Err("logger panicked".into());
    };

    // Channel for jobs (main -> workers)
    let (jobs_tx, jobs_rx) = mpsc::channel::<i32>();
    // Channel for results (workers -> main)
    let (results_tx, results_rx) = mpsc::channel::<(usize, i32, i32)>();

    // Share the single Receiver across workers
    let shared_rx = Arc::new(Mutex::new(jobs_rx));

    // Start a pool of 4 workers
    let workers = spawn_workers(4, Arc::clone(&shared_rx), results_tx);

    // Enqueue jobs 1..=20
    for x in 1..=20 {
        let Ok(()) = jobs_tx.send(x) else {
            eprintln!("[main] failed to send job {x}");
            break;
        };
    }
    // Close the job channel so workers know when to stop
    drop(jobs_tx);

    // Collect exactly 20 results (regardless of order)
    for _ in 0..20 {
        let Ok((wid, input, sq)) = results_rx.recv() else {
            eprintln!("[main] results channel closed early");
            break;
        };
        println!("[worker {wid}] {input}^2 = {sq}");
    }
    // Drop last results receiver clone on main side (optional; end of scope handles it)
    // drop(results_rx);

    // Join all workers
    for (i, h) in workers.into_iter().enumerate() {
        let Ok(_) = h.join() else {
            return Err(format!("worker {i} panicked").into());
        };
    }
    Ok(())
}
