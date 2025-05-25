use uuid::uuid;
use warp::mock::{
    mock_array_type_class, mock_bool_type_class, mock_char_type_class, mock_enum_type_class,
    mock_float_type_class, mock_function_type_class, mock_int_type_class, mock_ptr_type_class,
    mock_struct_type_class, mock_type, mock_type_metadata, mock_type_ref_type_class,
    mock_union_type_class,
};
use warp::r#type::guid::TypeGUID;
use warp::r#type::{Alignment, ComputedType, Type, TypeModifiers};

#[test]
fn test_type_creation() {
    let bool_type = mock_type("bool", mock_bool_type_class());
    let int_type = mock_type("int", mock_int_type_class(None, true));
    let char_type = mock_type("char", mock_char_type_class(None));
    let float_type = mock_type("float", mock_float_type_class(None));
    let array_type = mock_type("array", mock_array_type_class(&float_type, 3));
    let ptr_type = mock_type("ptr", mock_ptr_type_class(&int_type, None));
    let ref_type = mock_type("ref", mock_type_ref_type_class(&int_type));
    let func_type = mock_type("func", mock_function_type_class());
    let struct_type = mock_type(
        "struct",
        mock_struct_type_class(&[(0, "field_0", &bool_type), (32, "field_4", &array_type)]),
    );
    let union_type = mock_type(
        "union",
        mock_union_type_class(&[("member_0", &int_type), ("member_1", &array_type)]),
    );
    let enum_type = mock_type(
        "enum",
        mock_enum_type_class(&int_type, &[("item_0", 0), ("item_1", 1)]),
    );

    // Take all of these types and create the flatbuffer object for them, then check that the flatbuffer object is valid.
    let types = [
        bool_type,
        int_type,
        char_type,
        float_type,
        array_type,
        ptr_type,
        ref_type,
        func_type,
        struct_type,
        union_type,
        enum_type,
    ];
    for ty in types {
        let ty_bytes = ty.to_bytes();
        let ty_from_bytes = Type::from_bytes(&ty_bytes).expect("Valid type from bytes");
        assert_eq!(ty, ty_from_bytes);
    }
}

#[test]
fn test_type_sizing() {
    let int_type = mock_type("int", mock_int_type_class(None, true));
    assert_eq!(int_type.size(), None);
    let int32_type = mock_type("int32", mock_int_type_class(Some(32), true));
    assert_eq!(int32_type.size(), Some(32));
    let float16_type = mock_type("float16", mock_float_type_class(Some(16)));
    assert_eq!(float16_type.size(), Some(16));
    let char8 = mock_type("char8", mock_char_type_class(Some(8)));
    assert_eq!(char8.size(), Some(8));
    let array_type = mock_type("array", mock_array_type_class(&float16_type, 3));
    assert_eq!(array_type.size(), Some(16 * 3));
    let ptr64_type = mock_type("ptr64", mock_ptr_type_class(&int_type, Some(64)));
    assert_eq!(ptr64_type.size(), Some(64));
    let ref_type = mock_type("ref", mock_type_ref_type_class(&int_type));
    assert_eq!(ref_type.size(), None);
    let func_type = mock_type("func", mock_function_type_class());
    assert_eq!(func_type.size(), None);
    let struct_type = mock_type(
        "struct",
        mock_struct_type_class(&[(0, "field_0", &int32_type), (32, "field_4", &array_type)]),
    );
    assert_eq!(struct_type.size(), Some(32 + 16 * 3));
    let union_type = mock_type(
        "union",
        mock_union_type_class(&[("member_0", &int32_type), ("member_1", &array_type)]),
    );
    assert_eq!(union_type.size(), array_type.size());
    let enum_type = mock_type(
        "enum",
        mock_enum_type_class(&int32_type, &[("item_0", 0), ("item_1", 1)]),
    );
    assert_eq!(enum_type.size(), Some(32));
}

#[test]
fn test_type_metadata() {
    let mut int_type = mock_type("int", mock_int_type_class(None, true));
    // We should never implicitly store metadata.
    assert!(int_type.metadata.is_empty());
    int_type.metadata.push(mock_type_metadata("my_metadata"));
    assert_eq!(int_type.metadata.len(), 1);

    // Round-trip it through flatbuffers to make sure the metadata is kept.
    let ty_bytes = int_type.to_bytes();
    let ty_from_bytes = Type::from_bytes(&ty_bytes).expect("Valid type from bytes");
    assert_eq!(ty_from_bytes, int_type);
}

#[test]
fn test_type_modifiers() {
    let mut int_type = mock_type("int", mock_int_type_class(None, true));
    // We should never implicitly add modifiers.
    assert!(!int_type.is_const());
    assert!(!int_type.is_volatile());
    int_type.modifiers.set(TypeModifiers::Constant, true);
    assert!(int_type.is_const());
    int_type.modifiers.set(TypeModifiers::Volatile, true);
    assert!(int_type.is_volatile());

    // Round-trip it through flatbuffers to make sure the modifiers are kept.
    let ty_bytes = int_type.to_bytes();
    let ty_from_bytes = Type::from_bytes(&ty_bytes).expect("Valid type from bytes");
    assert_eq!(ty_from_bytes, int_type);
}

#[test]
fn test_type_alignment() {
    let mut int_type = mock_type("int", mock_int_type_class(None, true));
    // We should always be naturally aligned by default.
    assert_eq!(int_type.alignment, Alignment::Access);
    // Fixed with 8 bits is considered __packed.
    int_type.alignment = Alignment::Fixed(8);

    // Round-trip it through flatbuffers to make sure alignment is kept.
    let ty_bytes = int_type.to_bytes();
    let ty_from_bytes = Type::from_bytes(&ty_bytes).expect("Valid type from bytes");
    assert_eq!(ty_from_bytes, int_type);
}

#[test]
fn test_computed_type() {
    let int_type = mock_type("int", mock_int_type_class(None, true));
    let computed_int_type = ComputedType::new(int_type.clone());
    assert_eq!(computed_int_type.ty, int_type);
    let correct_type_guid = TypeGUID::from(uuid!("9f5eb7e0-986a-5af2-aa37-798b2bafae10"));
    assert_eq!(computed_int_type.guid, correct_type_guid);
}
