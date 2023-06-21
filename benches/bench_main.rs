use criterion::{black_box, criterion_group, criterion_main, Criterion};
use valve_kv_tools::{format_keyvalue, lint_keyvalue, serialize_keyvalue, FormatterConfig};

fn criterion_linter_benchmark(c: &mut Criterion) {
    let response = minreq::get("https://raw.githubusercontent.com/surftimer/SurfTimer/dev/addons/sourcemod/translations/surftimer.phrases.txt")
        .send().unwrap();
    let input = response.as_str().unwrap();
    c.bench_function("linter - surftimer.phrases", |b| {
        b.iter(|| {
            let _res = black_box(lint_keyvalue(input));
        })
    });
}

fn criterion_formatter_benchmark(c: &mut Criterion) {
    let response = minreq::get("https://raw.githubusercontent.com/surftimer/SurfTimer/dev/addons/sourcemod/translations/surftimer.phrases.txt")
        .send().unwrap();
    let input = response.as_str().unwrap();
    c.bench_function("formatter - surftimer.phrases", |b| {
        b.iter(|| {
            let _res = black_box(format_keyvalue(input, FormatterConfig::default()));
        })
    });
}

fn criterion_serializer_benchmark(c: &mut Criterion) {
    let response = minreq::get("https://raw.githubusercontent.com/surftimer/SurfTimer/dev/addons/sourcemod/translations/surftimer.phrases.txt")
        .send().unwrap();
    let input = response.as_str().unwrap();
    c.bench_function("serializer - surftimer.phrases", |b| {
        b.iter(|| {
            let _res = black_box(serialize_keyvalue(input));
        })
    });
}

criterion_group!(
    benches,
    criterion_linter_benchmark,
    criterion_formatter_benchmark,
    criterion_serializer_benchmark
);
criterion_main!(benches);
