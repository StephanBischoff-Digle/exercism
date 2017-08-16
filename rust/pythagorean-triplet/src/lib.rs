use std::thread;
use std::sync::mpsc;
use std::sync::Mutex;
use std::sync::Arc;

/// naive threaded brute force method
pub fn find() -> Option<u64> {
    let running = Arc::new(Mutex::new(true));

    let (tx, rx) = mpsc::channel();

    let num_threads = 4;
    let mut threads = vec![];

    for index in 0..num_threads {
        let (running, tx) = (running.clone(), tx.clone());
        threads.push(thread::spawn(move || {
            let delta = 1000u64 / num_threads;
            let lower = index * delta;
            let upper = (index + 1) * delta;
            let mut counter = 0u64;
            println!("thread {}: lower: {}, upper: {}", index, lower, upper);

            for a in lower..upper {
                for b in 1..1000 {
                    if !*running.lock().unwrap() {
                        println!(
                            "early return {}, tested {} datapoints ({:6.2}%)",
                            index,
                            counter,
                            (counter as f64) / (delta as f64 * 1_000_000f64) * 100f64
                        );
                        return;
                    }

                    if b == a {
                        continue;
                    }
                    for c in 1..1000 {
                        if c == a || c == b {
                            continue;
                        }
                        let sum = (b as u32).pow(2) + (c as u32).pow(2);
                        if sum == (a as u32).pow(2) && a + b + c == 1000 {
                            println!("a: {}\nb: {}\nc: {}", a, b, c);
                            let _ = tx.send(Some(a * b * c));
                            println!(
                                "{} returning, tried {} datapoints ({:6.2}%)",
                                index,
                                counter,
                                (counter as f64) / (delta as f64 * 1_000_000f64) * 100f64
                            );
                            return;
                        }
                        counter += 1;
                    }
                }
            }
        }));
    }

    let ret = rx.recv().unwrap_or_default();
    {
        let mut running = running.lock().unwrap();
        *running = false;
    }
    for t in threads {
        t.join().unwrap();
    }

    ret
}
