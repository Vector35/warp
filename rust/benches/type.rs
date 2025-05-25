use criterion::{criterion_group, criterion_main, Criterion};
use warp::mock::{mock_int_type_class, mock_type};
use warp::r#type::class::{StructureClass, StructureMember, TypeClass};
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

pub fn struct_benchmark(c: &mut Criterion) {
    let int_type = mock_type("my_int", mock_int_type_class(None, false));
    let structure_member = StructureMember::builder()
        .name("member")
        .ty(int_type)
        .offset(0)
        .build();
    let struct_class = StructureClass::new(vec![structure_member]);
    let struct_type = Type::builder()
        .name("my_struct".to_owned())
        .class(TypeClass::Structure(struct_class))
        .build();

    c.bench_function("uuid struct", |b| {
        b.iter(|| {
            let _ = TypeGUID::from(&struct_type);
        })
    });

    c.bench_function("computed struct", |b| b.iter(|| struct_type.to_bytes()));
}

criterion_group!(benches, void_benchmark, struct_benchmark);
criterion_main!(benches);
