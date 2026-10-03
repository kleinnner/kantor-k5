use criterion::{black_box, criterion_group, criterion_main, Criterion};
use k5_core::{k5_512, k2048, kof, DOMAIN_TAG};
use k5_core::sponge::K5Sponge;

fn bench_k5_512(c: &mut Criterion) {
    let data = vec![0x42u8; 65536];
    c.bench_function("K5-512 64KB", |b| {
        b.iter(|| k5_512(black_box(&data)))
    });
}

fn bench_k5_512_small(c: &mut Criterion) {
    let data = b"small input";
    c.bench_function("K5-512 11B", |b| {
        b.iter(|| k5_512(black_box(data)))
    });
}

fn bench_k2048(c: &mut Criterion) {
    let data = vec![0x42u8; 65536];
    c.bench_function("K2048 64KB", |b| {
        b.iter(|| k2048(black_box(&data)))
    });
}

fn bench_kof(c: &mut Criterion) {
    let data = vec![0x42u8; 65536];
    c.bench_function("KOF-8192 64KB", |b| {
        b.iter(|| kof(black_box(&data), 8192))
    });
}

fn bench_sponge_absorb(c: &mut Criterion) {
    let data = vec![0x42u8; 1048576];
    c.bench_function("Sponge absorb 1MB", |b| {
        b.iter(|| {
            let mut s = K5Sponge::new(DOMAIN_TAG);
            s.absorb(black_box(&data)).squeeze(512)
        })
    });
}

criterion_group!(benches, bench_k5_512, bench_k5_512_small, bench_k2048, bench_kof, bench_sponge_absorb);
criterion_main!(benches);
