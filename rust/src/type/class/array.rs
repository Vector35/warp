use bon::bon;
use std::hash::Hash;

use crate::cached_builder::CachedFlatBufferBuilder;
use crate::r#type::Type;
use crate::{fb_type as fb, FlatBufferObject};
use flatbuffers::WIPOffset;

// We re-export bit flags as there is no need to wrap them.
pub use fb::ArrayModifiers;

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayClass {
    pub length: Option<u64>,
    pub member_type: Box<Type>,
    pub modifiers: ArrayModifiers,
}

#[bon]
impl ArrayClass {
    #[builder]
    pub fn new(length: Option<u64>, member_type: Type, modifiers: Option<ArrayModifiers>) -> Self {
        Self {
            length,
            member_type: Box::new(member_type),
            modifiers: modifiers.unwrap_or_default(),
        }
    }
}

impl ArrayClass {
    pub fn size(&self) -> Option<u64> {
        Some(self.length? * self.member_type.size()?)
    }
}

impl FlatBufferObject for ArrayClass {
    type FbType<'fbb> = fb::Array<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let member_type = self.member_type.create(builder);
        fb::Array::create(
            builder,
            &fb::ArrayArgs {
                type_: Some(member_type),
                // NOTE: 0 means the array is dynamically sized.
                // TODO: Is this correct?
                length: self.length.unwrap_or(0),
                modifiers: self.modifiers,
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let class = Self {
            length: match value.length() {
                0 => None,
                len => Some(len),
            },
            member_type: Box::new(Type::from_object(&value.type_())?),
            modifiers: value.modifiers(),
        };

        Some(class)
    }
}

impl Eq for ArrayClass {}

impl Hash for ArrayClass {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.length.hash(state);
        self.member_type.hash(state);
        // NOTE: Flatbuffers currently do not add Hash impl for bitfields.
        self.modifiers.bits().hash(state);
    }
}
