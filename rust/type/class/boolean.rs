use crate::fb_type as fb;
use bon::Builder;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct BooleanClass {
    pub width: Option<u16>,
}

impl BooleanClass {
    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::Boolean<'a>> {
        fb::Boolean::create(
            builder,
            &fb::BooleanArgs {
                width: self.width.map(Into::into).as_ref(),
            },
        )
    }

    pub fn size(&self) -> Option<u64> {
        self.width.map(|w| w as u64)
    }
}

impl From<fb::Boolean<'_>> for BooleanClass {
    fn from(value: fb::Boolean<'_>) -> Self {
        Self {
            width: value.width().map(Into::into),
        }
    }
}

impl Distribution<BooleanClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BooleanClass {
        // TODO: Maybe we should restrict this to "normal" widths?
        BooleanClass { width: rng.gen() }
    }
}
