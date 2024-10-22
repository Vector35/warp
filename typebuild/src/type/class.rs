pub mod array;
pub mod boolean;
mod character;
pub mod enumeration;
mod float;
pub mod function;
pub mod integer;
pub mod pointer;
pub mod referrer;
pub mod structure;
pub mod union;
pub mod void;

pub use crate::r#type::class::array::ArrayClass;
pub use crate::r#type::class::boolean::BooleanClass;
pub use crate::r#type::class::character::CharacterClass;
pub use crate::r#type::class::enumeration::{EnumerationClass, EnumerationMember};
pub use crate::r#type::class::float::FloatClass;
pub use crate::r#type::class::function::{CallingConvention, FunctionClass, FunctionMember};
pub use crate::r#type::class::integer::IntegerClass;
pub use crate::r#type::class::pointer::PointerClass;
pub use crate::r#type::class::referrer::ReferrerClass;
pub use crate::r#type::class::structure::{StructureClass, StructureMember};
pub use crate::r#type::class::union::{UnionClass, UnionMember};
pub use crate::r#type::class::void::VoidClass;
use crate::Build;
use fbcg_rust::fb_type as fb;
use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeClass {
    // TODO: Make void class internally, make this Void no attached item.
    Void,
    // TOOD: Rename Boolean
    Bool(BooleanClass),
    Integer(IntegerClass),
    Character(CharacterClass),
    Float(FloatClass),
    Pointer(PointerClass),
    Array(ArrayClass),
    Structure(StructureClass),
    Enumeration(EnumerationClass),
    Union(UnionClass),
    Function(FunctionClass),
    Referrer(ReferrerClass),
}

impl TypeClass {
    pub fn ty(&self) -> fb::TypeClass {
        match self {
            TypeClass::Void => fb::TypeClass::Void,
            TypeClass::Bool(_) => fb::TypeClass::Boolean,
            TypeClass::Integer(_) => fb::TypeClass::Integer,
            TypeClass::Character(_) => fb::TypeClass::Character,
            TypeClass::Float(_) => fb::TypeClass::Float,
            TypeClass::Pointer(_) => fb::TypeClass::Pointer,
            TypeClass::Array(_) => fb::TypeClass::Array,
            TypeClass::Structure(_) => fb::TypeClass::Structure,
            TypeClass::Enumeration(_) => fb::TypeClass::Enumeration,
            TypeClass::Union(_) => fb::TypeClass::Union,
            TypeClass::Function(_) => fb::TypeClass::Function,
            TypeClass::Referrer(_) => fb::TypeClass::Referrer,
        }
    }
}

impl Build for TypeClass {
    type FBType<'a> = UnionWIPOffset;

    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Self::FBType<'a>> {
        match self {
            TypeClass::Void => VoidClass.create(builder).as_union_value(),
            TypeClass::Bool(c) => c.create(builder).as_union_value(),
            TypeClass::Integer(c) => c.create(builder).as_union_value(),
            TypeClass::Character(c) => c.create(builder).as_union_value(),
            TypeClass::Float(c) => c.create(builder).as_union_value(),
            TypeClass::Pointer(c) => c.create(builder).as_union_value(),
            TypeClass::Array(c) => c.create(builder).as_union_value(),
            TypeClass::Structure(c) => c.create(builder).as_union_value(),
            TypeClass::Enumeration(c) => c.create(builder).as_union_value(),
            TypeClass::Union(c) => c.create(builder).as_union_value(),
            TypeClass::Function(c) => c.create(builder).as_union_value(),
            TypeClass::Referrer(c) => c.create(builder).as_union_value(),
        }
    }

    fn size(&self) -> Option<u64> {
        match self {
            TypeClass::Void => VoidClass.size(),
            TypeClass::Bool(c) => c.size(),
            TypeClass::Integer(c) => c.size(),
            TypeClass::Character(c) => c.size(),
            TypeClass::Float(c) => c.size(),
            TypeClass::Pointer(c) => c.size(),
            TypeClass::Array(c) => c.size(),
            TypeClass::Structure(c) => c.size(),
            TypeClass::Enumeration(c) => c.size(),
            TypeClass::Union(c) => c.size(),
            TypeClass::Function(c) => c.size(),
            TypeClass::Referrer(c) => c.size(),
        }
    }
}
impl Distribution<TypeClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TypeClass {
        // TODO: We need more types than Standard to randomly generate structures.
        match rng.gen_range(0..15) {
            0 => TypeClass::Void,
            1 => TypeClass::Bool(rng.gen()),
            2 => TypeClass::Integer(rng.gen()),
            3 => TypeClass::Character(rng.gen()),
            4 => TypeClass::Float(rng.gen()),
            5 => TypeClass::Array(rng.gen()),
            // 6 => TypeClass::Structure(rng.gen()),
            7 => TypeClass::Enumeration(rng.gen()),
            // 8 => TypeClass::Union(rng.gen()),
            // 9 => TypeClass::Function(rng.gen()),
            10 => TypeClass::Referrer(rng.gen()),
            // Pointer is weighted so that we get more nesting.
            _ => TypeClass::Pointer(rng.gen()),
        }
    }
}

impl From<BooleanClass> for TypeClass {
    fn from(value: BooleanClass) -> Self {
        Self::Bool(value)
    }
}

impl From<IntegerClass> for TypeClass {
    fn from(value: IntegerClass) -> Self {
        Self::Integer(value)
    }
}

impl From<CharacterClass> for TypeClass {
    fn from(value: CharacterClass) -> Self {
        Self::Character(value)
    }
}

impl From<FloatClass> for TypeClass {
    fn from(value: FloatClass) -> Self {
        Self::Float(value)
    }
}

impl From<PointerClass> for TypeClass {
    fn from(value: PointerClass) -> Self {
        Self::Pointer(value)
    }
}

impl From<ArrayClass> for TypeClass {
    fn from(value: ArrayClass) -> Self {
        Self::Array(value)
    }
}

impl From<StructureClass> for TypeClass {
    fn from(value: StructureClass) -> Self {
        Self::Structure(value)
    }
}

impl From<FunctionClass> for TypeClass {
    fn from(value: FunctionClass) -> Self {
        Self::Function(value)
    }
}

impl From<ReferrerClass> for TypeClass {
    fn from(value: ReferrerClass) -> Self {
        Self::Referrer(value)
    }
}
impl From<UnionClass> for TypeClass {
    fn from(value: UnionClass) -> Self {
        Self::Union(value)
    }
}

impl From<EnumerationClass> for TypeClass {
    fn from(value: EnumerationClass) -> Self {
        Self::Enumeration(value)
    }
}
