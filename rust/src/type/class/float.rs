use crate::cached_builder::CachedFlatBufferBuilder;
use crate::{fb_type as fb, FlatBufferObject};
use bon::Builder;
use flatbuffers::WIPOffset;
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct FloatClass {
    /// Width in bits
    pub width: Option<u16>,
}

impl FloatClass {
    pub fn size(&self) -> Option<u64> {
        self.width.map(|w| w as u64)
    }
}

impl FlatBufferObject for FloatClass {
    type FbType<'fbb> = fb::Float<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        fb::Float::create(
            builder,
            &fb::FloatArgs {
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
