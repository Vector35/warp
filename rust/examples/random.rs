use std::env;
use warp::chunk::{Chunk, ChunkKind, CompressionType};
use warp::mock::{mock_function, mock_function_type_class, mock_type};
use warp::r#type::chunk::TypeChunk;
use warp::signature::chunk::SignatureChunk;
use warp::WarpFileHeader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <count>", args[0]);
        std::process::exit(1);
    }
    let count: u32 = args[1].parse().expect("Valid integer");

    // Fill out a signature chunk with functions.
    let mut functions = Vec::new();
    for i in 0..count {
        functions.push(mock_function(&format!("function_{}", i)));
    }
    let _signature_chunk = SignatureChunk::new(&functions).expect("Failed to create chunk");
    let signature_chunk = Chunk::new(
        ChunkKind::Signature(_signature_chunk),
        CompressionType::Zstd,
    );
    println!("Created signature chunk with {} functions...", count);

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
    println!("Created type chunk with {} types...", types.len());

    let file = warp::WarpFile::new(WarpFileHeader::new(), vec![signature_chunk, type_chunk]);
    println!("Created file with {} chunks...", file.chunks.len());

    std::fs::write("random.warp", file.to_bytes()).expect("Failed to write file");
}
