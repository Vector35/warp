use criterion::{criterion_group, criterion_main, Criterion};
use std::str::FromStr;
use warp::chunk::{Chunk, ChunkKind, CompressionType};
use warp::mock::{mock_function, mock_function_type_class, mock_type};
use warp::r#type::chunk::TypeChunk;
use warp::signature::chunk::SignatureChunk;
use warp::signature::function::FunctionGUID;
use warp::{WarpFile, WarpFileHeader};

pub fn chunk_benchmark(c: &mut Criterion) {
    let count = 10000;
    // Fill out a signature chunk with functions.
    let mut functions = Vec::new();
    for i in 0..count {
        functions.push(mock_function(&format!("function_{}", i)));
    }
    let mut _signature_chunk = SignatureChunk::new(&functions).expect("Failed to create chunk");
    let signature_chunk = Chunk::new(
        ChunkKind::Signature(_signature_chunk.clone()),
        CompressionType::None,
    );

    // Fill out a type chunk with types.
    let mut types = Vec::new();
    for i in 0..count {
        types.push(mock_type(
            &format!("type_{}", i),
            mock_function_type_class(),
        ));
    }
    let _type_chunk = TypeChunk::new(&types).expect("Failed to create chunk");
    let type_chunk = Chunk::new(ChunkKind::Type(_type_chunk), CompressionType::Zstd);
    let file = WarpFile::new(
        WarpFileHeader::new(),
        vec![signature_chunk.clone(), type_chunk],
    );
    c.bench_function("file to bytes", |b| {
        b.iter(|| {
            file.to_bytes();
        })
    });

    let known_function = functions.get(326).expect("Failed to get function 326").guid;
    c.bench_function("find known function", |b| {
        b.iter(|| {
            _signature_chunk
                .raw_functions_with_guid(&known_function)
                .count()
        })
    });

    let unknown_function = FunctionGUID::from_str("467aae0d-84d4-4804-90d2-a62159502367")
        .expect("Failed to get unknown function GUID");
    c.bench_function("find unknown function", |b| {
        b.iter(|| {
            _signature_chunk
                .raw_functions_with_guid(&unknown_function)
                .count()
        })
    });
}

criterion_group!(benches, chunk_benchmark);
criterion_main!(benches);
