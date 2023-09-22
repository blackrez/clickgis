use std::io::sink;

use clickgis::handle;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let function = black_box("st_aswkt");
    let input = black_box(b"0101000000cade52ce17d32740bd35b05582814b40");
    let input_100 = black_box(b"0101000000cade52ce17d32740bd35b05582814b40\n".repeat(100));
    let mut output = sink();
    c.bench_function("handle 1", |b| {
        b.iter(|| handle(function, &input[..], &mut output))
    });
    c.bench_function("handle 100", |b| {
        b.iter(|| handle(function, &input_100[..], &mut output))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
