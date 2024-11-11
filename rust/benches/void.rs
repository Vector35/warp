use criterion::{criterion_group, criterion_main, Criterion};
use warp::r#type::class::TypeClass;
use warp::r#type::guid::TypeGUID;
use warp::r#type::Type;

pub fn void_benchmark(c: &mut Criterion) {
    let void_type = Type::builder()
        .name("my_void".to_owned())
        .class(TypeClass::Void)
        .build();

    c.bench_function("uuid void", |b| {
        b.iter(|| {
            let _ = TypeGUID::from(&void_type);
        })
    });

    c.bench_function("computed void", |b| b.iter(|| void_type.to_bytes()));
}

criterion_group!(benches, void_benchmark);
criterion_main!(benches);
