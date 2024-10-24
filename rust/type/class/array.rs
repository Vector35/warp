use bon::bon;

use crate::fb_type as fb;
use crate::r#type::Type;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use rand::distributions::Standard;
use rand::prelude::*;

// We re-export bit flags as there is no need to wrap them.
pub use fb::ArrayModifiers;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayClass {
    pub length: Option<u64>,
    pub member_type: Type,
    pub modifiers: ArrayModifiers,
}

#[bon]
impl ArrayClass {
    #[builder]
    pub fn new(length: Option<u64>, member_type: Type, modifiers: Option<ArrayModifiers>) -> Self {
        Self {
            length,
            member_type,
            modifiers: modifiers.unwrap_or_default(),
        }
    }
}

impl ArrayClass {
    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::Array<'a>> {
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

    pub fn size(&self) -> Option<u64> {
        Some(self.length? * self.member_type.size()?)
    }
}

impl From<fb::Array<'_>> for ArrayClass {
    fn from(value: fb::Array<'_>) -> Self {
        Self {
            length: match value.length() {
                0 => None,
                len => Some(len),
            },
            member_type: value.type_().into(),
            modifiers: value.modifiers(),
        }
    }
}

impl Distribution<ArrayClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ArrayClass {
        let mut modifiers = ArrayModifiers::empty();
        // 50% chance array is null terminated.
        modifiers.set(ArrayModifiers::NullTerminated, rng.gen_bool(0.5));
        ArrayClass {
            length: rng.gen(),
            member_type: rng.gen(),
            modifiers,
        }
    }
}
