use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pricer::{call};

fn simple_benchmark(c: &mut Criterion) {
    c.bench_function("american_call_option", |b| {
        b.iter(|| {
            let t = 1.0;
            let s = 100.0;
            let k = 110.0;
            let r = 0.05;
            let sigma = 0.25;
            let q = 0.03;
            let n = 100;
            call(
                black_box(t), 
                black_box(s), 
                black_box(k), 
                black_box(r), 
                black_box(sigma), 
                black_box(q), 
                black_box(n)
            )
        })
    });
}


criterion_group!(benches, simple_benchmark);
criterion_main!(benches);
