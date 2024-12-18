use crate::fb_type as fb;
use bon::Builder;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct CharacterClass {
    /// Width in bits
    pub width: Option<u16>,
}

impl CharacterClass {
    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::Character<'a>> {
        fb::Character::create(
            builder,
            &fb::CharacterArgs {
                width: self.width.map(Into::into).as_ref(),
            },
        )
    }

    pub fn size(&self) -> Option<u64> {
        self.width.map(|w| w as u64)
    }
}

impl From<fb::Character<'_>> for CharacterClass {
    fn from(value: fb::Character<'_>) -> Self {
        Self {
            width: value.width().map(Into::into),
        }
    }
}

impl Distribution<CharacterClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CharacterClass {
        // 90% chance this type will have a width.
        let width = match rng.gen_bool(0.9) {
            true => Some(rng.gen_range(1..=256)),
            false => None,
        };
        CharacterClass { width }
    }
}
