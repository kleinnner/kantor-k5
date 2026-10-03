use std::time::Instant;
use k5_core::{k5_512, k2048, kof};

fn main() {
    let data = vec![0x42u8; 65536];

    let start = Instant::now();
    let n = 100;
    for _ in 0..n { k5_512(&data); }
    let dur = start.elapsed();
    let mbps = (data.len() as f64 * n as f64 / dur.as_secs_f64()) / 1e6;
    println!("K5-512  64KB  x{}: {:6.2?}  {:.0} MB/s", n, dur, mbps);

    let start = Instant::now();
    for _ in 0..n { k2048(&data); }
    let dur = start.elapsed();
    let mbps = (data.len() as f64 * n as f64 / dur.as_secs_f64()) / 1e6;
    println!("K2048   64KB  x{}: {:6.2?}  {:.0} MB/s", n, dur, mbps);

    let n_small = 10000;
    let small = b"hello";
    let start = Instant::now();
    for _ in 0..n_small { k5_512(small); }
    let dur = start.elapsed();
    let ops = n_small as f64 / dur.as_secs_f64();
    println!("K5-512  x{}: {:6.2?}  {:.0} ops/s", n_small, dur, ops);

    let start = Instant::now();
    for _ in 0..n { kof(&data, 8192); }
    let dur = start.elapsed();
    let mbps = (data.len() as f64 * n as f64 / dur.as_secs_f64()) / 1e6;
    println!("KOF-8192 64KB x{}: {:6.2?}  {:.0} MB/s", n, dur, mbps);
}
