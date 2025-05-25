use warp::chunk::{Chunk, ChunkKind, CompressionType};
use warp::mock::mock_function;
use warp::signature::chunk::SignatureChunk;
use warp::{WarpFile, WarpFileHeader};

#[test]
fn test_file_creation() {
    // Construct a signature chunk to check for on file creation.
    let func_0 = mock_function("func_0");
    let func_1 = mock_function("func_1");
    let func_2 = mock_function("func_2");
    let funcs = vec![func_0, func_1, func_2];
    let signature_chunk = SignatureChunk::new(&funcs).expect("Failed to create signature chunk");
    let chunk_kind = ChunkKind::Signature(signature_chunk.clone());
    let chunk = Chunk::new(chunk_kind.clone(), CompressionType::None);
    let compressed_chunk = Chunk::new(chunk_kind, CompressionType::Zstd);
    let chunks = vec![chunk, compressed_chunk];

    // Create the file and check the contents.
    let file_header = WarpFileHeader::new();
    let file = WarpFile::new(file_header, chunks);
    assert_eq!(file.header.version, 1);
    assert_eq!(file.chunks.len(), 2);
    assert_eq!(
        file.chunks[0].kind,
        ChunkKind::Signature(signature_chunk.clone())
    );
    assert_eq!(
        file.chunks[1].kind,
        ChunkKind::Signature(signature_chunk.clone())
    );

    // Read the file back and check the contents.
    let file_bytes = file.to_bytes();
    let file_read = WarpFile::from_bytes(&file_bytes).expect("Failed to read file");
    assert_eq!(file_read.header.version, 1);
    assert_eq!(
        file_read.chunks.len(),
        2,
        "Both uncompressed and compressed chunk should have been read"
    );
    assert_eq!(
        file_read.chunks[0].kind,
        ChunkKind::Signature(signature_chunk.clone())
    );
    assert_eq!(
        file_read.chunks[1].kind,
        ChunkKind::Signature(signature_chunk.clone())
    );
}

#[test]
fn test_file_format_regression() {
    // Test that the "../fixtures/random.warp" file can be read.
    // This file was created with a previous version of Warp and should still be readable.
    // If this test fails, it means the format has changed in a backwards incompatible way.
    let file_bytes = include_bytes!("../fixtures/random.warp");
    let _file_read =
        WarpFile::from_bytes(file_bytes).expect("Failed to read file, format must have changed!");
    insta::assert_debug_snapshot!(_file_read);
}
