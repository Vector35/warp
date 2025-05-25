use crate::cached_builder::CachedFlatBufferBuilder;
use crate::{fb_type as fb, FlatBufferObject};
use bon::Builder;
use flatbuffers::WIPOffset;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct CharacterClass {
    /// Width in bits
    pub width: Option<u16>,
}

impl CharacterClass {
    pub fn size(&self) -> Option<u64> {
        self.width.map(|w| w as u64)
    }
}

impl FlatBufferObject for CharacterClass {
    type FbType<'fbb> = fb::Character<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        fb::Character::create(
            builder,
            &fb::CharacterArgs {
                width: self.width.map(Into::into).as_ref(),
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        Some(Self {
            width: value.width().map(Into::into),
        })
    }
}
