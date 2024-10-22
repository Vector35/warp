use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub mod guid;
pub mod r#type;

pub mod prelude {
    pub use crate::guid::TypeGUID;
    pub use crate::r#type::class::{
        ArrayClass, BooleanClass, CallingConvention, CharacterClass, EnumerationClass,
        EnumerationMember, FloatClass, FunctionClass, FunctionMember, IntegerClass, PointerClass,
        ReferrerClass, StructureClass, StructureMember, TypeClass, UnionClass, UnionMember,
        VoidClass,
    };
    pub use crate::r#type::modifier::{
        DescriptorModifierClass, MetadataModifierClass, TypeModifier, TypeModifierClass,
    };
    pub use crate::r#type::{Alignment, ComputedType, Type};
    pub use crate::Build;
}

// TODO: Remove this trait, its useless.
pub trait Build {
    type FBType<'a>;
    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Self::FBType<'a>>;
    // If None we must reason about the size using a tuple of (class, architecture, platform), and other context to infer the size...
    fn size(&self) -> Option<u64>;
}
