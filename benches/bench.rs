use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rustline::server::runtime::run;
use rustline::tests::load_test::run_load_test;
use tokio::runtime::Runtime;

fn http_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    rt.spawn(async {
        run("127.0.0.1:8080".to_string()).await.unwrap();
    });

    let mut group = c.benchmark_group("http_server");

    for connections in [100, 500, 1000] {
        group.bench_with_input(
            BenchmarkId::new("concurrent_requests", connections),
            &connections,
            |b, &conns| {
                b.to_async(&rt)
                    .iter(|| async { run_load_test(conns).await.unwrap() });
            },
        );
    }
}

criterion_group!(benches, http_benchmark);
criterion_main!(benches);
