use warp::r#type::class::{
    ArrayClass, BooleanClass, EnumerationClass, EnumerationMember, FunctionClass, FunctionMember,
    IntegerClass, PointerClass, StructureClass, StructureMember, TypeClass, UnionClass,
    UnionMember,
};
use warp::r#type::Type;

fn main() {
    let void_type = Type::builder()
        .name("my_void".to_owned())
        .class(TypeClass::Void)
        .build();
    dbg!(&void_type);

    let bool_class = BooleanClass::builder().width(8).build();
    let bool_type = Type::builder()
        .name("my_bool".to_owned())
        .class(bool_class)
        .build();
    dbg!(&bool_type);

    let int_class = IntegerClass::builder().width(8).signed(false).build();
    let int_type = Type::builder()
        .name("my_int".to_owned())
        .class(int_class)
        .build();
    dbg!(&int_type);

    let ptr_class = PointerClass::builder()
        .width(8)
        .child_type(void_type.clone())
        .build();
    let ptr_type = Type::builder()
        .name("my_ptr".to_owned())
        .class(ptr_class)
        .build();
    dbg!(&ptr_type);

    let array_class = ArrayClass::builder()
        .length(4)
        .member_type(void_type.clone())
        .build();
    let array_type = Type::builder()
        .name("my_array".to_owned())
        .class(array_class)
        .build();
    dbg!(&array_type);

    let enum_class = EnumerationClass::new(
        int_type.clone(),
        vec![
            EnumerationMember::builder().name("one").constant(1).build(),
            EnumerationMember::builder().name("two").constant(2).build(),
            EnumerationMember::builder()
                .name("five")
                .constant(5)
                .build(),
            EnumerationMember::builder().constant(6).build(),
        ],
    );
    let enum_type = Type::builder()
        .name("my_enum".to_owned())
        .class(enum_class)
        .build();
    dbg!(&enum_type);

    let union_class = UnionClass::new(vec![
        UnionMember::new("int".to_owned(), int_type.clone()),
        UnionMember::new("bool".to_owned(), bool_type.clone()),
        UnionMember::new("ptr".to_owned(), ptr_type.clone()),
        UnionMember::new("enum".to_owned(), enum_type.clone()),
        UnionMember::new("array".to_owned(), array_type.clone()),
    ]);
    let union_type = Type::builder()
        .name("my_union".to_owned())
        .class(union_class)
        .build();
    dbg!(&union_type);

    let struct_class = StructureClass::new(vec![
        StructureMember::builder()
            .name("int")
            .ty(int_type.clone())
            .offset(0)
            .build(),
        StructureMember::builder()
            .name("bool")
            .ty(bool_type.clone())
            .offset(64)
            .build(),
        StructureMember::builder()
            .name("pointer")
            .ty(ptr_type.clone())
            .offset(72)
            .build(),
        StructureMember::builder()
            .name("enum")
            .ty(enum_type.clone())
            .offset(136)
            .build(),
        StructureMember::builder()
            .name("array")
            .ty(array_type.clone())
            .offset(200)
            .build(),
    ]);
    let struct_type = Type::builder()
        .name("my_struct".to_owned())
        .class(struct_class)
        .build();
    dbg!(&struct_type);

    let func_class = FunctionClass::new(
        None,
        vec![
            FunctionMember::builder()
                .name("param_0")
                .ty(int_type.clone())
                .build(),
            FunctionMember::builder()
                .name("param_1")
                .ty(bool_type.clone())
                .build(),
        ],
        vec![],
    );
    let func_type = Type::builder()
        .name("my_func".to_owned())
        .class(func_class)
        .build();
    dbg!(&func_type);
}
