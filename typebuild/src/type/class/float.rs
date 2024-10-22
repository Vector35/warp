use crate::Build;
use bon::Builder;
use fbcg_rust::fb_type as fb;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct FloatClass {
    /// Width in bits
    pub width: Option<u16>,
}

impl From<fb::Float<'_>> for FloatClass {
    fn from(value: fb::Float<'_>) -> Self {
        Self {
            width: value.width().map(Into::into),
        }
    }
}

impl Build for FloatClass {
    type FBType<'a> = fb::Float<'a>;

    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Self::FBType<'a>> {
        fb::Float::create(
            builder,
            &fb::FloatArgs {
                width: self.width.map(Into::into).as_ref(),
            },
        )
    }

    fn size(&self) -> Option<u64> {
        self.width.map(|w| w as u64)
    }
}

impl Distribution<FloatClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FloatClass {
        // 90% chance this type will have a width.
        let width = match rng.gen_bool(0.9) {
            true => Some(rng.gen_range(1..=256)),
            false => None,
        };
        FloatClass { width }
    }
}
