use std::{fs::File, io::BufReader};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut reader = black_box({
        let file = File::open("test-file.txt").unwrap();
        BufReader::new(file)
    });

    c.bench_function("yeetus", |b| b.iter(|| rust_sandbox::yeet(&mut reader)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
