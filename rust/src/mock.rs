//! Mocking functions to help with testing

use crate::r#type::class::function::{Location, StackLocation};
use crate::r#type::class::{
    ArrayClass, BooleanClass, CharacterClass, EnumerationClass, EnumerationMember, FloatClass,
    FunctionClass, IntegerClass, PointerClass, ReferrerClass, StructureClass, StructureMember,
    TypeClass, UnionClass, UnionMember,
};
use crate::r#type::guid::TypeGUID;
use crate::r#type::{Type, TypeMetadata};
use crate::signature::comment::FunctionComment;
use crate::signature::constraint::{Constraint, ConstraintGUID};
use crate::signature::function::{Function, FunctionGUID};
use crate::signature::variable::FunctionVariable;
use crate::symbol::{Symbol, SymbolClass};

/// Computes the function guid from the given magic string.
pub fn mock_function_guid(magic: &str) -> FunctionGUID {
    FunctionGUID::from(magic.as_bytes())
}

pub fn mock_constraint_guid(magic: &str) -> ConstraintGUID {
    ConstraintGUID::from(magic.as_bytes())
}

pub fn mock_constraint(magic: &str, offset: Option<i64>) -> Constraint {
    Constraint {
        guid: mock_constraint_guid(magic),
        offset,
    }
}

pub fn mock_symbol(magic: &str, class: SymbolClass) -> Symbol {
    Symbol {
        name: magic.to_string(),
        modifiers: Default::default(),
        class,
    }
}

pub fn mock_function_type_class() -> TypeClass {
    TypeClass::Function(FunctionClass {
        calling_convention: None,
        in_members: vec![],
        out_members: vec![],
    })
}

pub fn mock_int_type_class(width: Option<u16>, signed: bool) -> TypeClass {
    TypeClass::Integer(IntegerClass { width, signed })
}

pub fn mock_bool_type_class() -> TypeClass {
    TypeClass::Boolean(BooleanClass { width: None })
}

pub fn mock_void_type_class() -> TypeClass {
    TypeClass::Void
}

pub fn mock_char_type_class(width: Option<u16>) -> TypeClass {
    TypeClass::Character(CharacterClass { width })
}

pub fn mock_float_type_class(width: Option<u16>) -> TypeClass {
    TypeClass::Float(FloatClass { width })
}

pub fn mock_array_type_class(member_ty: &Type, len: u64) -> TypeClass {
    TypeClass::Array(ArrayClass {
        length: Some(len),
        member_type: Box::new(member_ty.clone()),
        modifiers: Default::default(),
    })
}

pub fn mock_struct_type_class(members: &[(u64, &str, &Type)]) -> TypeClass {
    TypeClass::Structure(StructureClass {
        members: members
            .iter()
            .map(|&(offset, name, ty)| StructureMember {
                name: Some(name.to_string()),
                offset,
                ty: Box::new(ty.clone()),
                modifiers: Default::default(),
            })
            .collect(),
    })
}

pub fn mock_enum_type_class(member_ty: &Type, members: &[(&str, u64)]) -> TypeClass {
    TypeClass::Enumeration(EnumerationClass {
        member_type: Box::new(member_ty.clone()),
        members: members
            .iter()
            .map(|&(name, constant)| EnumerationMember {
                name: Some(name.to_string()),
                constant,
            })
            .collect(),
    })
}

pub fn mock_union_type_class(members: &[(&str, &Type)]) -> TypeClass {
    TypeClass::Union(UnionClass {
        members: members
            .iter()
            .map(|&(name, ty)| UnionMember {
                name: name.to_string(),
                ty: Box::new(ty.clone()),
            })
            .collect(),
    })
}

pub fn mock_ptr_type_class(child_ty: &Type, width: Option<u16>) -> TypeClass {
    TypeClass::Pointer(PointerClass {
        width,
        child_type: Box::new(child_ty.clone()),
        addressing: Default::default(),
    })
}

pub fn mock_ref_type_class(guid: Option<TypeGUID>, name: Option<String>) -> TypeClass {
    TypeClass::Referrer(ReferrerClass { guid, name })
}

pub fn mock_guid_ref_type_class(guid: TypeGUID) -> TypeClass {
    mock_ref_type_class(Some(guid), None)
}

pub fn mock_name_ref_type_class(name: String) -> TypeClass {
    mock_ref_type_class(None, Some(name))
}

pub fn mock_type_ref_type_class(ref_ty: &Type) -> TypeClass {
    let guid = TypeGUID::from(ref_ty);
    let name = ref_ty.name.clone();
    mock_ref_type_class(Some(guid), name)
}

pub fn mock_type(magic: &str, class: TypeClass) -> Type {
    Type {
        name: Some(magic.to_string()),
        class,
        confidence: 255,
        modifiers: Default::default(),
        metadata: vec![],
        alignment: Default::default(),
        ancestors: vec![],
    }
}

pub fn mock_type_metadata(magic: &str) -> TypeMetadata {
    TypeMetadata::new_string(magic.to_string(), magic.to_string())
}

pub fn mock_function_comment(magic: &str) -> FunctionComment {
    FunctionComment {
        offset: 0,
        text: magic.to_string(),
    }
}

pub fn mock_function_variable(magic: &str) -> FunctionVariable {
    FunctionVariable {
        offset: 0,
        location: Location::Stack(StackLocation { offset: 0 }),
        name: Some(magic.to_string()),
        ty: None,
    }
}

pub fn mock_function(magic: &str) -> Function {
    Function {
        guid: mock_function_guid(magic),
        symbol: mock_symbol(magic, SymbolClass::Function),
        ty: Some(mock_type(magic, mock_function_type_class())),
        constraints: Default::default(),
        comments: Default::default(),
        variables: Default::default(),
    }
}
