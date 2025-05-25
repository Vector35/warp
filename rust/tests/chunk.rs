use warp::chunk::{Chunk, ChunkHandler, ChunkHeader, ChunkKind, ChunkType, CompressionType};
use warp::mock::{
    mock_constraint, mock_enum_type_class, mock_float_type_class, mock_function,
    mock_int_type_class, mock_type,
};
use warp::r#type::chunk::TypeChunk;
use warp::r#type::guid::TypeGUID;
use warp::signature::chunk::SignatureChunk;
use warp::target::Target;

#[test]
fn test_signature_chunk() {
    let function_0 = mock_function("function_0");
    let function_1 = mock_function("function_1");
    let function_2 = mock_function("function_2");
    let functions = vec![function_0, function_1, function_2];
    let chunk = SignatureChunk::new(&functions).expect("Failed to create signature chunk");
    for function in functions {
        let chunk_functions: Vec<_> = chunk.functions_with_guid(&function.guid).collect();
        assert_eq!(chunk_functions.len(), 1);
        assert_eq!(chunk_functions[0], function);
    }
}

#[test]
fn test_type_chunk() {
    let type_0 = mock_type("type_0", mock_int_type_class(None, false));
    let type_1 = mock_type("type_1", mock_float_type_class(None));
    let type_2 = mock_type("type_2", mock_enum_type_class(&type_0, &[("member", 0x10)]));
    let types = vec![type_0, type_1, type_2];
    let chunk = TypeChunk::new(&types).expect("Failed to create type chunk");
    for ty in types {
        let guid = TypeGUID::from(&ty);
        let chunk_ty = chunk.type_with_guid(&guid).expect("Failed to get type");
        assert_eq!(chunk_ty, ty);
    }
}

#[test]
fn test_chunk_header() {
    let signature_chunk = SignatureChunk::new(&vec![]).expect("Failed to create signature chunk");
    let chunk_kind = ChunkKind::Signature(signature_chunk);
    let chunk_header =
        ChunkHeader::from_chunk_kind(&chunk_kind, CompressionType::None, Target::default());
    assert_eq!(chunk_header.chunk_type, ChunkType::Signatures);
    assert_eq!(chunk_header.version, chunk_kind.version());
    assert_eq!(chunk_header.compression_type, CompressionType::None);
    assert_eq!(chunk_header.size, 24);
}

#[test]
fn test_chunk_compression() {
    let signature_chunk = SignatureChunk::new(&vec![]).expect("Failed to create signature chunk");
    let chunk_kind = ChunkKind::Signature(signature_chunk);
    let chunk_header =
        ChunkHeader::from_chunk_kind(&chunk_kind, CompressionType::Zstd, Target::default());
    assert_eq!(chunk_header.compression_type, CompressionType::Zstd);
    assert_eq!(
        chunk_header.size, 24,
        "This should be the size of the uncompressed chunk"
    );
    let data = chunk_kind.as_bytes();

    // Compress the data using the chunk header's compression type.
    let encoded_data = chunk_header
        .encode_data(&data)
        .expect("Failed to encode chunk data");
    assert_eq!(
        encoded_data.len(),
        22,
        "This should be the size of the compressed chunk"
    );

    // Decompress the data using the chunk header's compression type.
    let decoded_data = chunk_header
        .decode_data(&encoded_data)
        .expect("Failed to decode chunk data");
    assert_eq!(decoded_data, data);
}

#[test]
fn test_type_chunk_merging() {
    let type_0 = mock_type("type_0", mock_int_type_class(None, false));
    let type_1 = mock_type("type_1", mock_float_type_class(None));
    let type_2 = mock_type("type_2", mock_enum_type_class(&type_0, &[("member", 0x10)]));
    let types_3 = vec![type_0.clone(), type_1.clone(), type_2];
    let chunk_0 = TypeChunk::new(&types_3).expect("Failed to create type chunk");

    let types_2 = vec![type_0, type_1];
    let chunk_1 = TypeChunk::new(&types_2).expect("Failed to create type chunk");

    let type_3 = mock_type("type_3", mock_float_type_class(None));
    let types_1 = vec![type_3];
    let chunk_2 = TypeChunk::new(&types_1).expect("Failed to create type chunk");

    let merged_chunk =
        TypeChunk::merge(&[chunk_0, chunk_1, chunk_2]).expect("Failed to merge chunks");
    let merged_chunk_types: Vec<_> = merged_chunk.types().collect();
    assert_eq!(
        merged_chunk_types.len(),
        4,
        "Merging should eliminate duplicates"
    );
}

#[test]
fn test_signature_chunk_merging() {
    let function_0 = mock_function("function_0");
    let function_1 = mock_function("function_1");
    let function_2 = mock_function("function_2");
    let functions_3 = vec![function_0.clone(), function_1.clone(), function_2.clone()];
    let chunk_0 = SignatureChunk::new(&functions_3).expect("Failed to create signature chunk");

    // Add a constraint to duplicate function_0 so we can make sure it is kept.
    let mut function_0_with_constraint = function_0.clone();
    function_0_with_constraint
        .constraints
        .insert(mock_constraint("constraint_0", None));
    let functions_2 = vec![function_0_with_constraint.clone(), function_1];
    let chunk_1 = SignatureChunk::new(&functions_2).expect("Failed to create signature chunk");

    let function_3 = mock_function("function_3");
    let functions_1 = vec![function_3];
    let chunk_2 = SignatureChunk::new(&functions_1).expect("Failed to create signature chunk");

    let merged_chunk = SignatureChunk::merge(&[chunk_0, chunk_1, chunk_2.clone()])
        .expect("Failed to merge chunks");
    let merged_chunk_funcs: Vec<_> = merged_chunk.functions().collect();
    assert_eq!(
        merged_chunk_funcs.len(),
        4,
        "Merging should eliminate duplicates"
    );

    // We should have function_0 with the constraint, if it is missing, then the merging failed to preserve constraints.
    merged_chunk_funcs
        .into_iter()
        .find(|f| f == &function_0_with_constraint)
        .expect("Failed to find function_0 with constraint");

    // Make sure functions with differing types are not merged.
    let mut unique_function_3 = mock_function("function_3");
    unique_function_3.ty = Some(mock_type("test", mock_int_type_class(None, true)));
    let unique_functions_1 = vec![unique_function_3];
    let unique_chunk_2 =
        SignatureChunk::new(&unique_functions_1).expect("Failed to create signature chunk");

    let merged_chunk =
        SignatureChunk::merge(&[chunk_2.clone(), unique_chunk_2]).expect("Failed to merge chunks");
    let merged_chunk_funcs: Vec<_> = merged_chunk.functions().collect();
    assert_eq!(
        merged_chunk_funcs.len(),
        2,
        "Merging should keep same functions with different types"
    );

    // Make sure functions without a type are merged.
    let mut untyped_function_3 = mock_function("function_3");
    untyped_function_3.ty = None;
    let untyped_functions_1 = vec![untyped_function_3];
    let untyped_chunk_2 =
        SignatureChunk::new(&untyped_functions_1).expect("Failed to create signature chunk");

    let merged_chunk = SignatureChunk::merge(&[
        untyped_chunk_2.clone(),
        chunk_2.clone(),
        untyped_chunk_2.clone(),
    ])
    .expect("Failed to merge chunks");
    let merged_chunk_funcs: Vec<_> = merged_chunk.functions().collect();
    assert_eq!(
        merged_chunk_funcs.len(),
        1,
        "Merging should merge same functions if missing type"
    );
    assert!(
        merged_chunk_funcs[0].ty.is_some(),
        "Merged function missing type"
    );

    // Make sure functions with the same name but different GUID are not merged.
    let mut guid_2_function_3 = mock_function("function_3");
    guid_2_function_3.guid = function_2.guid.clone();
    let guid_2_functions_1 = vec![guid_2_function_3];
    let guid_2_chunk_2 =
        SignatureChunk::new(&guid_2_functions_1).expect("Failed to create signature chunk");

    let merged_chunk = SignatureChunk::merge(&[
        guid_2_chunk_2.clone(),
        chunk_2,
        untyped_chunk_2,
        guid_2_chunk_2,
    ])
    .expect("Failed to merge chunks");
    let merged_chunk_funcs: Vec<_> = merged_chunk.functions().collect();
    println!("{:#?}", merged_chunk_funcs);
    assert_eq!(
        merged_chunk_funcs.len(),
        2,
        "Merging should merge same functions if same guid"
    );
}

#[test]
fn test_chunk_merging() {
    // Test to make sure that chunks with different targets do not merge.
    let function_0 = mock_function("function_0");
    let function_1 = mock_function("function_1");
    let function_2 = mock_function("function_2");
    let functions_3 = vec![function_0.clone(), function_1.clone(), function_2];
    let signature_chunk_0 =
        SignatureChunk::new(&functions_3).expect("Failed to create signature chunk");
    let signature_chunk_1 =
        SignatureChunk::new(&functions_3).expect("Failed to create signature chunk");
    let signature_chunk_2 =
        SignatureChunk::new(&functions_3).expect("Failed to create signature chunk");
    let chunk_0 = Chunk::new(
        ChunkKind::Signature(signature_chunk_0),
        CompressionType::None,
    );
    // Different compressions are ignored since we adjust the compression type when merging.
    let chunk_1 = Chunk::new(
        ChunkKind::Signature(signature_chunk_1),
        CompressionType::Zstd,
    );
    let chunk_2 = Chunk::new_with_target(
        ChunkKind::Signature(signature_chunk_2),
        CompressionType::None,
        Target {
            architecture: Some("hexagon".to_string()),
            platform: None,
        },
    );
    let merged_chunks = Chunk::merge(&[chunk_0, chunk_1, chunk_2], CompressionType::None);
    println!("{:#?}", merged_chunks);
    assert_eq!(merged_chunks.len(), 2);
}
