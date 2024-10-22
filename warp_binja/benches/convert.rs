use binaryninja::binaryview::BinaryViewExt;
use binaryninja::headless::Session;
use binaryninja::types::Conf;
use criterion::{criterion_group, criterion_main, Criterion};
use warp_binja::convert::from_bn_type;

pub fn type_conversion_benchmark(c: &mut Criterion) {
    let session = Session::new();
    let bv = session.load(env!("TEST_BIN_LIBRARY_OBJ")).unwrap();
    let functions = bv.functions();
    assert_eq!(functions.len(), 6);
    let mut function_iter = functions.into_iter();
    let first_function = function_iter.next().unwrap();
    let first_function_ty = Conf::new(first_function.function_type(), u8::MAX);

    // TODO: Add a macro benchmark.

    c.bench_function("type conversion first function", |b| {
        b.iter(|| {
            from_bn_type(
                &bv,
                first_function_ty.contents.clone(),
                first_function_ty.confidence,
            );
        })
    });

    c.bench_function("type conversion all functions", |b| {
        b.iter(|| {
            for func in &functions {
                from_bn_type(&bv, func.function_type(), u8::MAX);
            }
        })
    });

    c.bench_function("type conversion all types", |b| {
        b.iter(|| {
            for ty in &bv.types() {
                from_bn_type(&bv, ty.type_object().clone(), u8::MAX);
            }
        })
    });
}

criterion_group!(benches, type_conversion_benchmark);
criterion_main!(benches);
